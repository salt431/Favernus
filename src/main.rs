extern crate vulkano;
extern crate winit;
extern crate smallvec;

use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateFlags, physical::PhysicalDevice};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use vulkano::VulkanLibrary;
use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use winit::window::WindowBuilder;
use smallvec::smallvec;
use vulkano::device::QueueCreateInfo;
use vulkano::NonExhaustive;
use std::sync::{Arc, Mutex, Weak};
use std::sync::mpsc::{channel, Sender};
use smallvec::alloc::fmt::DebugStruct;



#[repr(C)] // Ensure similar memory layout
struct MockNonExhaustive {
    _private: (), // Use a private field to prevent direct usage
}



struct Favernus {
    instance: Arc<Instance>, 
    physical_device: Arc<PhysicalDevice>,
    surface: Arc<Surface>,
    device: Arc<Device>,
    queue: Arc<Queue>, // Queue is wrapped in an Arc 
}

impl Favernus {
    fn new() -> Result<Self, Box<dyn Error>> {
        let nonex: NonExhaustive = unsafe { std::mem::transmute(MockNonExhaustive { _private: () }) };
        let vulkan_lib = VulkanLibrary::new()?;
        let window = WindowBuilder::new().build(&EventLoop::new())?;
        let instance = Instance::new(vulkan_lib, InstanceCreateInfo::default())?;
        let surface = Surface::from_window(instance.clone(), window.into())?;
        let physical_device = instance
            .enumerate_physical_devices()?
            .next()
            .ok_or("No physical device available")?;

        let extensions = DeviceExtensions { khr_swapchain: true, ..DeviceExtensions::empty() };
        let queues_family = 0; 

        let queue_create_info = QueueCreateInfo {
            queue_family_index: queues_family,
            flags: QueueCreateFlags::empty(),
            queues: vec![0.5],
            _ne: nonex, // Assuming the library allows `None` as a placeholder for the `_ne` field 
        };

        let device_create_info = DeviceCreateInfo {
            queue_create_infos: vec![queue_create_info],
            enabled_extensions: extensions,
            enabled_features: Features::empty(),
            physical_devices: smallvec![physical_device.clone()],
            private_data_slot_request_count: 0,
            _ne: nonex, // Assuming the library allows `None` as a placeholder for the `_ne` field 
        };

        let (device, mut queue_iter) = Device::new(physical_device.clone(), device_create_info)?;
        let queue = queue_iter.next().expect("Failed to create a queue");
        Ok(Self {
            instance,
            physical_device,
            surface,
            device,
            queue,
        })
    }


    fn run(&mut self) {
        // Ensure 'self' reference remains valid within the method
        let event_loop_ref = EventLoop::new();
    
        event_loop_ref.run(move |event, _, control_flow| {
            // Handle events without direct reference to 'self'
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit; 
                }
                
                // Add more event handling logic here
                _ => (),
            }
        });
    }

    fn clear_and_draw(&self) {
        // ... (Vulkan code for clearing and drawing)
        println!("Clearing and drawing...");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut favernus = Favernus::new()?;
    favernus.run();
    Ok(())
}
