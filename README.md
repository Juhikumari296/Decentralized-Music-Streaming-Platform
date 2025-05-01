# Decentralized Music Streaming Platform

## Project Title
**Decentralized Music Streaming Platform**

## Project Description
The Decentralized Music Streaming Platform is a blockchain-based solution that allows artists to upload their music tracks and listeners to stream them securely. This platform eliminates intermediaries, ensures transparency in payments, and empowers artists to directly control their content.

## Project Vision
The vision of this project is to revolutionize the music streaming industry by utilizing decentralized technology, ensuring fair compensation for artists, and providing listeners with a transparent and censorship-resistant platform.

## Key Features
- **Track Creation**: Artists can create and upload their music tracks, making them available for streaming.
- **Track Streaming**: Listeners can stream tracks after a successful payment, which is handled transparently on the blockchain.
- **Track Removal**: Artists can remove or unpublish their tracks when necessary, marking them as unavailable.
- **Payment Transparency**: Every transaction made for streaming a track is recorded on the blockchain, ensuring transparency.

## Contract Details

### Contract Address: CAI6RAFOIKCJ2G4NEWL5TPJ2UOOC4KSWGCRPOCGYH64B7OYRXL6NDPWB

The `MusicStreamingContract` provides the following key functionalities:
1. **Create Track**: Artists can create and upload their music tracks. The track includes details like the title, album, duration, and price to listen.
   - **Function**: `create_track`
   - **Inputs**: `artist`, `title`, `album`, `duration`, `price`
   - **Outputs**: `track_id`

2. **Stream Track**: Listeners can stream tracks by making a payment. The payment process is simulated for simplicity in this contract.
   - **Function**: `stream_track`
   - **Inputs**: `listener`, `track_id`
   - **Outputs**: `bool`

3. **Remove Track**: Artists can mark their tracks as unavailable for various reasons, effectively removing them from the platform.
   - **Function**: `remove_track`
   - **Inputs**: `artist`, `track_id`
   - **Outputs**: `bool`

4. **Get Track**: Retrieve the details of a specific track.
   - **Function**: `get_track`
   - **Inputs**: `track_id`
   - **Outputs**: `MusicTrack`
