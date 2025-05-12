use dotenv::dotenv;
use futures::StreamExt;
use livekit::prelude::*;
use livekit::webrtc::audio_stream::native::NativeAudioStream;
use std::env;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let url = env::var("LIVEKIT_URL").expect("LIVEKIT_URL must be set");
    let token = env::var("LIVEKIT_TOKEN").expect("LIVEKIT_TOKEN must be set");

    // Connect to LiveKit server
    let (room, mut room_events) = Room::connect(&url, &token, RoomOptions::default()).await?;
    info!("Connected to room: {}", room.name());

    // Listen for track subscriptions
    tokio::spawn(async move {
        while let Some(event) = room_events.recv().await {
            match event {
                RoomEvent::TrackSubscribed {
                    participant, track, ..
                } => {
                    info!(
                        "Subscribed to track: {} from {}",
                        track.sid(),
                        participant.identity()
                    );

                    // Match on the RemoteTrack enum to handle audio tracks
                    match track {
                        RemoteTrack::Audio(audio_track) => {
                            let rtc_track = audio_track.rtc_track();
                            let sample_rate = 48000;
                            let num_channels = 2;
                            let mut audio_stream =
                                NativeAudioStream::new(rtc_track, sample_rate, num_channels);
                            tokio::spawn(async move {
                                // Receive the audio frames in a new task
                                while let Some(audio_frame) = audio_stream.next().await {
                                    log::info!("received audio frame - {audio_frame:#?}");
                                }
                            });
                        }
                        _ => {
                            info!("Received non-audio track; ignoring.");
                        }
                    }
                }
                _ => {}
            }
        }
    });

    // Keep process running
    tokio::signal::ctrl_c().await?;
    Ok(())
}
