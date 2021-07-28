use crate::vk::util::{QueueFamilyIndices, SurfaceData};

use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

pub struct SwapchainData {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

pub fn create_swapchain(
    instance: &ash::Instance,
    device: &ash::Device,
    window: &winit::window::Window,
    physical_device: vk::PhysicalDevice,
    surface_data: &SurfaceData,
    queue_family: &QueueFamilyIndices,
) -> SwapchainData {
    let support = get_swapchain_support(physical_device, &surface_data);

    let surface_format = choose_swapchain_format(&support.formats);
    let present_mode = choose_swapchain_present_mode(&support.present_modes);
    let extent = choose_swapchain_extent(&support.capabilities, window);

    let image_count = support.capabilities.min_image_count + 1;
    let image_count = if support.capabilities.max_image_count > 0 {
        image_count.min(support.capabilities.max_image_count)
    } else {
        image_count
    };

    let (image_sharing_mode, queue_family_index_count, queue_family_indices) =
        if queue_family.graphics_family != queue_family.present_family {
            (
                vk::SharingMode::CONCURRENT,
                2,
                vec![
                    queue_family.graphics_family.unwrap(),
                    queue_family.present_family.unwrap(),
                ],
            )
        } else {
            (vk::SharingMode::EXCLUSIVE, 0, vec![])
        };

    let create_info = vk::SwapchainCreateInfoKHR {
        surface: surface_data.surface,
        min_image_count: image_count,

        image_format: surface_format.format,
        image_color_space: surface_format.color_space,
        image_extent: extent,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode,

        p_queue_family_indices: queue_family_indices.as_ptr(),
        queue_family_index_count,
        pre_transform: support.capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,

        present_mode,
        clipped: vk::TRUE,
        ..Default::default()
    };

    let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
    let swapchain = unsafe {
        swapchain_loader
            .create_swapchain(&create_info, None)
            .expect("Failed to create swapchain.")
    };

    let images = unsafe {
        swapchain_loader
            .get_swapchain_images(swapchain)
            .expect("Failed to get swapchain images")
    };

    SwapchainData {
        swapchain_loader,
        swapchain,
        format: surface_format.format,
        extent: extent,
        images,
    }
}

struct SwapchainSupportDetails {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

fn get_swapchain_support(
    physical_device: vk::PhysicalDevice,
    surface_data: &SurfaceData,
) -> SwapchainSupportDetails {
    unsafe {
        let capabilities = surface_data
            .surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface_data.surface)
            .expect("Failed to get device surface capabilities.");

        let formats = surface_data
            .surface_loader
            .get_physical_device_surface_formats(physical_device, surface_data.surface)
            .expect("Failed to get device surface formats.");

        let present_modes = surface_data
            .surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface_data.surface)
            .expect("Failed to get device surface present modes.");

        SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        }
    }
}

fn choose_swapchain_format(available_formats: &Vec<vk::SurfaceFormatKHR>) -> vk::SurfaceFormatKHR {
    for available_format in available_formats {
        if available_format.format == vk::Format::B8G8R8A8_SRGB
            && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            return available_format.clone();
        }
    }

    return available_formats.first().unwrap().clone();
}

fn choose_swapchain_present_mode(
    available_present_modes: &Vec<vk::PresentModeKHR>,
) -> vk::PresentModeKHR {
    for &available_present_mode in available_present_modes.iter() {
        if available_present_mode == vk::PresentModeKHR::MAILBOX {
            return available_present_mode;
        }
    }

    vk::PresentModeKHR::FIFO
}

fn choose_swapchain_extent(
    capabilities: &vk::SurfaceCapabilitiesKHR,
    window: &winit::window::Window,
) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::max_value() {
        capabilities.current_extent
    } else {
        use num::clamp;

        let window_size = window.inner_size();

        vk::Extent2D {
            width: clamp(
                window_size.width as u32,
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ),
            height: clamp(
                window_size.height as u32,
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ),
        }
    }
}

pub fn create_swapchain_image_views(
    device: &ash::Device,
    format: vk::Format,
    images: &Vec<vk::Image>,
) -> Vec<vk::ImageView> {
    images
        .iter()
        .map(|&image| {
            let create_info = vk::ImageViewCreateInfo {
                view_type: vk::ImageViewType::TYPE_2D,
                format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                image,
                ..Default::default()
            };
            unsafe {
                device
                    .create_image_view(&create_info, None)
                    .expect("Failed to create image view.")
            }
        })
        .collect()
}

pub fn create_framebuffers(
    device: &ash::Device,
    render_pass: vk::RenderPass,
    image_views: &Vec<vk::ImageView>,
    swapchain_extent: vk::Extent2D,
) -> Vec<vk::Framebuffer> {
    let mut framebuffers = vec![];

    for &image_view in image_views.iter() {
        let attachments = [image_view];

        let framebuffer_create_info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            width: swapchain_extent.width,
            height: swapchain_extent.height,
            layers: 1,
        };

        let framebuffer = unsafe {
            device
                .create_framebuffer(&framebuffer_create_info, None)
                .expect("Failed to create Framebuffer!")
        };

        framebuffers.push(framebuffer);
    }

    framebuffers
}
