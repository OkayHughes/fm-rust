

#![feature(globs)]

extern crate portaudio;

use portaudio::*;

fn main() -> () {

    let stream = init_portaudio_with_defaults_f32(1i32, 44100., 5u32);

    let notes = [1760f32, 1567.98f32, 1396.91f32, 1318.51f32, 1174.66f32, 
                1046.50f32, 987.77f32, 880f32, 783.99f32, 698.46f32, 659.26f32, 
                587.33f32, 523.25f32, 493.88f32, 440f32, 392f32, 349.23f32, 
                329.63f32, 293.66f32, 261.63f32, 246.94f32, 220f32, 196f32, 
                174.61f32, 164.81f32, 146.83f32, 130.81f32, 123.47f32, 110f32, 
                98f32, 87.31f32, 82.41f32, 73.42f32, 65.41f32, 61.74f32, 55f32, 49f32, 
                43.65f32, 41.20f32, 36.71f32, 32.70f32, 30.87];
    for i in notes.iter(){
        play_sine_wave(stream, *i as f32, 0.4, 44100., 5u32);
    }

    terminate_portaudio_f32(stream);

}

fn init_portaudio_with_defaults_f32(channel_count: i32, sample_rate: f64, buffer_size: u32) -> pa::PaStream<f32>{
    println!("Portaudio init error : {}", pa::get_error_text(pa::initialize()));

    let host_count = pa::get_host_api_count();
    println!("Portaudio host count : {}", host_count as int);

    let default_host = pa::get_default_host_api();
    println!("Portaudio default host : {}", default_host as int);

    let host_info = pa::get_host_api_info(default_host);
    println!("Portaudio host name : {}", host_info.unwrap().name);

    println!("Portaudio type id : {}",
             pa::host_api_type_id_to_host_api_index(types::PaCoreAudio) as int);


    let def_input = pa::get_default_input_device();


    if pa::get_device_info(def_input).is_none() {
       println!("error");
    }

    // PaStream test :
    let stream_params  = types::PaStreamParameters {
        device : def_input,
        channel_count : 2,
        sample_format : types::PaFloat32,
        suggested_latency : pa::get_device_info(def_input).unwrap().default_low_input_latency
    };

    let def_output = pa::get_default_output_device();
    println!("name : {}", pa::get_device_info(def_output).unwrap().name);

    let stream_params_out = types::PaStreamParameters {
        device : def_output,
        channel_count : channel_count,
        sample_format : types::PaFloat32,
        suggested_latency : pa::get_device_info(def_output).unwrap().default_low_output_latency
    };

    let mut stream : pa::PaStream<f32> = pa::PaStream::new(types::PaFloat32);

    let mut err = stream.open(Some(&stream_params), Some(&stream_params_out), sample_rate, buffer_size, types::PaClipOff);
    
    println!("Portaudio Start error : {}", pa::get_error_text(err));

    err = stream.start();

    stream
}

fn play_sine_wave(stream: pa::PaStream<f32>, frequency: f32, duration: f32,  sample_rate: f32,  buffer_size : u32) {
    //contiguous counter
    let mut x = 0i;
    for _ in range(0i, (duration * sample_rate / buffer_size as f32).round() as int){
        let mut buffer : Vec<f32> = Vec::with_capacity(buffer_size as uint);
        for _ in range(0u, buffer_size as uint){
            x += 1;
            let val = (x as f32 * frequency * 2. * std::num::Float::pi() / sample_rate).sin() * amplitude_function(duration, sample_rate, x);
            buffer.push(val);
        }
        let mut test = stream.get_stream_write_available();
        while test == 0{
            test = stream.get_stream_write_available();
        }

        stream.write(buffer, buffer_size);
    }
}
// uses 10* 2^-x * atan(x), x in radians.
fn amplitude_function(duration : f32, sample_rate: f32, pos : int ) -> f32 {
    let x = 1. * ((pos as f32 / (sample_rate * duration))-1.);
    10f32 * 2f32.powf(-1. * x) * x.atan()
}

fn terminate_portaudio_f32(mut stream : pa::PaStream<f32>){
    let mut err = types::PaNotInitialized;
    err = stream.close();
    println!("Portaudio Close stream error : {}", pa::get_error_text(err));

    println!("");


    println!("Portaudio terminate error : {}", pa::get_error_text(pa::terminate()));
}