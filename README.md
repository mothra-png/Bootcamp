# Stellar Movie Watchlist Smart Contract DApp

**Stellar Movie Watchlist Smart Contract DApp** - Blockchain-Based Decentralized Movie Tracking System

## Project Description

This project is a simple decentralized movie watchlist application built using Soroban smart contract on Stellar. The purpose of this app is to help users store and manage a list of movies they want to watch directly on the blockchain.

Each movie has a unique ID, a title, and a status that indicates whether the movie has been watched or not.

## Project Vision

Our vision is to improve how users manage personal entertainment lists in a decentralized way by:

- Decentralizing Data: Storing movie watchlists on a blockchain instead of centralized platforms
- Ensuring Ownership: Giving users full control over their personal watchlist data
- Guaranteeing Transparency: All actions are recorded and verifiable on the blockchain
- Enhancing Security: Preventing unauthorized modification of stored data
- Building Trustless Systems: Ensuring data integrity through smart contract logic

We aim to create a simple but reliable system where users can track their movie lists securely and independently.

## Key Features

### 1. **Add Movie**

- Add movies to your personal watchlist
- Each movie includes a title and a unique ID
- Automatically sets the movie as "not watched"
- Stored permanently on the blockchain

### 2. **View Movie List**

- Retrieve all stored movies in one function
- Displays movie title and watched status
- Easy to integrate with frontend applications

### 3. **Mark Movie as Watched**

- Update movie status to "watched"
- Based on unique movie ID
- Helps track which movies are completed

### 4. **Delete Movie**

- Remove a movie from the watchlist
- Uses unique ID for accuracy
- Keeps storage clean and organized

### 5. **Blockchain-Based Storage**

- All data is stored on Stellar blockchain
- Transparent and tamper-proof system
- Fast and low-cost transactions using Soroban

## Contract Details

- Contract Address: b3bd9a65a90d23e746aad37ede5ba07844fe39d9813e3e210398939421cc5761
  ![alt text](<img width="1919" height="967" alt="image" src="https://github.com/user-attachments/assets/05d4b9b1-09d1-4eae-bd75-1d6a014f4c3a" />
)

## Future Scope

### Short-Term Enhancements

1. **Input Validation**: Improve validation for movie titles
2. **Search Feature**: Allow users to search movies by title
3. **Filter System**: Separate watched and unwatched movies
4. **Better UI Integration**: Connect with a simple frontend interface

### Medium-Term Development

5. **User-Based Watchlist**: Separate movie lists for different users
6. **Rating System**: Allow users to rate movies
7. **Favorite Movies**: Add a feature to mark favorite movies
8. **History Tracking**: Track when a movie was added or watched

### Long-Term Vision

9. **Cross-Platform Integration**: Sync with external movie databases
10. **Decentralized Frontend**: Host UI on IPFS
11. **Recommendation System**: Suggest movies based on user activity
12. **AI Integration**: Provide smart movie recommendations

### Enterprise Features

13. **Shared Watchlist**: Allow multiple users to collaborate
14. **Analytics Dashboard**: Show user watching patterns
15. **Content Categorization**: Group movies by genre
16. **Multi-Language Support**: Support global users

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the main functions:

add_movie() → Add a new movie to the watchlist
get_movies() → Retrieve all stored movies
mark_watched() → Mark a movie as watched
delete_movie() → Remove a movie by its ID

---

**Stellar Movie Watchlist Smart Contract DApp** - Blockchain-Based Decentralized Movie Tracking System
