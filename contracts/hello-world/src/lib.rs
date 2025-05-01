#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address, Vec};

// MusicTrack structure to store track details
#[contracttype]
#[derive(Clone, PartialEq)]
pub struct MusicTrack {
    pub track_id: u64,
    pub artist: Address,
    pub title: String,
    pub album: String,
    pub duration: u64,   // Duration of track in seconds
    pub price: u64,      // Price to listen to the track (in smallest unit of currency)
    pub status: TrackStatus
}

// TrackStatus to manage track state
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum TrackStatus {
    Available,    // Track is available for streaming
    Unavailable,  // Track is unavailable (could be deleted or unpublished)
}

// For mapping track_id to MusicTrack
#[contracttype]
pub enum MusicTrackKey {
    Track(u64)
}

// Constants
const TRACK_COUNT: Symbol = symbol_short!("T_COUNT");

// Music Streaming Contract
#[contract]
pub struct MusicStreamingContract;

#[contractimpl]
impl MusicStreamingContract {
    // Create a new music track
    pub fn create_track(
        env: Env, 
        artist: Address, 
        title: String, 
        album: String, 
        duration: u64, 
        price: u64
    ) -> u64 {
        // Authenticate the artist
        artist.require_auth();
        
        // Validate input
        if duration == 0 || price == 0 {
            log!(&env, "Invalid track parameters");
            panic!("Invalid track parameters");
        }
        
        // Get current track count and increment
        let mut track_count: u64 = env.storage().instance().get(&TRACK_COUNT).unwrap_or(0);
        track_count += 1;
        
        // Create new music track
        let track = MusicTrack {
            track_id: track_count,
            artist: artist.clone(),
            title,
            album,
            duration,
            price,
            status: TrackStatus::Available,
        };
        
        // Store the track
        env.storage().instance().set(&MusicTrackKey::Track(track_count), &track);
        
        // Update track count
        env.storage().instance().set(&TRACK_COUNT, &track_count);
        
        log!(&env, "Music track created with ID: {}", track_count);
        
        track_count
    }

    // Stream a track
    pub fn stream_track(env: Env, listener: Address, track_id: u64) -> bool {
        // Authenticate the listener
        listener.require_auth();
        
        // Get the track
        let track = Self::get_track(env.clone(), track_id);
        
        // Validate track
        if track.track_id == 0 || track.status != TrackStatus::Available {
            log!(&env, "Track not available or does not exist");
            return false;
        }
        
        // Simulate a payment (for simplicity, assume payment is successful)
        log!(&env, "Track {} streamed by {}", track_id, listener);
        
        true
    }

    // Mark a track as unavailable (for removal or other reasons)
    pub fn remove_track(env: Env, artist: Address, track_id: u64) -> bool {
        // Authenticate the artist
        artist.require_auth();
        
        // Get the track
        let mut track = Self::get_track(env.clone(), track_id);
        
        // Validate track
        if track.track_id == 0 || track.status == TrackStatus::Unavailable {
            log!(&env, "Track does not exist or already removed");
            return false;
        }
        
        // Ensure the caller is the artist who created the track
        if track.artist != artist {
            log!(&env, "Only the artist can remove this track");
            return false;
        }
        
        // Mark track as unavailable
        track.status = TrackStatus::Unavailable;
        
        // Store the updated track
        env.storage().instance().set(&MusicTrackKey::Track(track_id), &track);
        
        log!(&env, "Track {} marked as unavailable", track_id);
        
        true
    }
    
    // Get track details
    pub fn get_track(env: Env, track_id: u64) -> MusicTrack {
        let key = MusicTrackKey::Track(track_id);
        
        env.storage().instance().get(&key).unwrap_or(MusicTrack {
            track_id: 0,
            artist: Address::from_str(&env, ""),
            title: String::from_str(&env, ""),
            album: String::from_str(&env, ""),
            duration: 0,
            price: 0,
            status: TrackStatus::Unavailable,
        })
    }
}
