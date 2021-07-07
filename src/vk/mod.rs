mod consts;
mod util;
mod platforms;
mod swapchain;
mod renderpass;

use crate::vk::util::*;
use crate::vk::consts::*;
use crate::vk::swapchain::create_swapchain;

use ash::version::DeviceV1_0;
// use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

struct SyncObjects {
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
}

pub struct VulkanApp {
    window: winit::window::Window,
    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,

    _physical_device: vk::PhysicalDevice,
    device: ash::Device,

    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    _swapchain_images: Vec<vk::Image>,
    _swapchain_format: vk::Format,
    _swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>,
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    sync: SyncObjects,
    current_frame: usize,
}

impl VulkanApp {
    pub fn new(event_loop: &EventLoop<()>) -> VulkanApp {
        let window = util::init_window(
            event_loop, 
            consts::WINDOW_TITLE, 
            consts::WINDOW_WIDTH, 
            consts::WINDOW_HEIGHT
        );

        let entry = unsafe { ash::Entry::new().unwrap() };
        let instance = util::create_instance(&entry, APP_NAME);
        
        let surface_data =
            create_surface(
                &entry,
                &instance,
                &window,
                WINDOW_WIDTH,
                WINDOW_HEIGHT
            );
        
        let physical_device = util::pick_physical_device(&instance, &surface_data);
        let (device, family_indices) = util::create_logical_device(
            &instance,
            physical_device,
            &surface_data
        );
        
        let graphics_queue = 
            unsafe { device.get_device_queue(family_indices.graphics_family.unwrap(), 0) };
        let present_queue = 
            unsafe { device.get_device_queue(family_indices.present_family.unwrap(), 0) };
        
        let swapchain_data = create_swapchain(
            &instance, 
            &device,
            &window,
            physical_device,
            &surface_data,
            &family_indices
        );
        
        let swapchain_imageviews = swapchain::create_swapchain_image_views(
            &device,
            swapchain_data.format,
            &swapchain_data.images
        );
        
        let render_pass = renderpass::create_render_pass(&device, swapchain_data.format);
        
        let (graphics_pipeline, pipeline_layout) = 
            util::create_graphics_pipeline(&device, render_pass, swapchain_data.extent);
        
        let swapchain_framebuffers = 
            swapchain::create_framebuffers(
                &device,
                render_pass,
                &swapchain_imageviews,
                swapchain_data.extent
            );
        
        let command_pool =
            util::create_command_pool(&device, &family_indices);
        
        let command_buffers =
            util::create_command_buffers(
                &device,
                command_pool,
                graphics_pipeline,
                &swapchain_framebuffers,
                render_pass,
                swapchain_data.extent
                );
        
        let sync =
            VulkanApp::create_sync_objects(&device);

        VulkanApp {
            window: window,
            _entry: entry,
            instance,
            surface_loader: surface_data.surface_loader,
            surface: surface_data.surface,
            
            _physical_device: physical_device,
            device,
            
            graphics_queue,
            present_queue,
            
            swapchain_loader: swapchain_data.swapchain_loader,
            swapchain: swapchain_data.swapchain,
            _swapchain_images: swapchain_data.images,
            _swapchain_format: swapchain_data.format,
            _swapchain_extent: swapchain_data.extent,
            
            swapchain_imageviews,
            swapchain_framebuffers,
            
            render_pass,
            graphics_pipeline,
            pipeline_layout,
            
            command_buffers,
            command_pool,

            sync,
            current_frame: 0
        }
    }

    fn draw_frame(&mut self) {
        let wait_fences = [self.sync.in_flight_fences[self.current_frame]];

        let (image_index, _is_sub_optimal) = unsafe {
            self.device
                .wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failet to wait for Fence!");

            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    std::u64::MAX,
                    self.sync.image_available_semaphores[self.current_frame],
                    vk::Fence::null(),
                )
                .expect("Failed to acquire next image!")
        };

        let wait_semaphores = [self.sync.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.sync.render_finished_semaphores[self.current_frame]];

        let submit_infos = [vk::SubmitInfo {
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_buffers[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            ..Default::default()
        }];

        unsafe {
            self.device
                .reset_fences(&wait_fences)
                .expect("Failed to reset Fence!");

            self.device
                .queue_submit(
                    self.graphics_queue,
                    &submit_infos,
                    self.sync.in_flight_fences[self.current_frame],
                )
                .expect("Failed to execute queue submit!");
        }

        let swapchains = [self.swapchain];

        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            ..Default::default()
        };

        unsafe {
            self.swapchain_loader
                .queue_present(self.present_queue, &present_info)
                .expect("Failed to execute queue present!");
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }
    
    fn create_sync_objects(device: &ash::Device) -> SyncObjects {
        let mut sync_objects = SyncObjects {
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            in_flight_fences: vec![],
        };

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();

        let fence_create_info = vk::FenceCreateInfo {
            flags: vk::FenceCreateFlags::SIGNALED,
            ..Default::default()
        };

        for _ in 0..consts::MAX_FRAMES_IN_FLIGHT {
            unsafe {
                let image_available_semaphore = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let render_finished_semaphore = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let inflight_fence = device
                    .create_fence(&fence_create_info, None)
                    .expect("Failed to create Fence Object!");

                sync_objects
                    .image_available_semaphores
                    .push(image_available_semaphore);
                sync_objects
                    .render_finished_semaphores
                    .push(render_finished_semaphore);
                sync_objects.in_flight_fences.push(inflight_fence);
            }
        }

        sync_objects
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            Event::RedrawRequested(_window_id) => {
                self.draw_frame();
            }
            Event::LoopDestroyed => {
                unsafe {
                    self.device
                        .device_wait_idle()
                        .expect("Failed to device wait idle!")
                };
            }
            _ => (),
        })
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            for _ in 0..MAX_FRAMES_IN_FLIGHT {}
            self.instance.destroy_instance(None);
        }
    }
}
