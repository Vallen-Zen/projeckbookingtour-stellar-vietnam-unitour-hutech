ProjectBookingTour Stellar - Vietnam UniTour Hutech
Project Description
ProjectBookingTour Stellar - Vietnam UniTour Hutech is a decentralized Real-World Asset (RWA) management system built on the Stellar blockchain using Soroban smart contracts. The platform enables tour operators and educational institutions to securely create, manage, and verify tour bookings on-chain.

Designed specifically for university tours (UniTour) and student events at Hutech, this system ensures transparent pricing, real-time seat availability, and immutable booking records. It allows authorized administrators to manage tour data while providing users (students/travelers) with a reliable, low-cost, and high-speed method to secure their spots.

Project Vision
The vision of this project is to modernize the tourism and educational event booking industry by leveraging blockchain technology to tokenize real-world assets (tour seats). By utilizing Stellar and Soroban, the platform aims to eliminate double-booking fraud, reduce intermediary fees, and create a transparent, tamper-resistant ecosystem for students and travel agencies in Vietnam.

Key Features
Tour Creation & Management: Authorized administrators can easily create new tours, set prices, and define total seat capacities.

Decentralized Booking System: Users can securely book available seats directly on-chain using their Stellar wallets.

Real-time Seat Tracking: The smart contract automatically updates and tracks available seats, preventing overbooking.

Role-Based Access Control: Strict cryptographic authorization ensures that only designated administrators can initialize and modify core tour parameters.

Immutable Storage: All booking records and tour details are stored permanently on the blockchain, ensuring auditability and trust.

Transparent Queries: Anyone can seamlessly query the blockchain to retrieve specific tour information or verify user bookings.

Usage Instructions
Deploy Contract: Deploy the smart contract to the Stellar network (Testnet/Futurenet).

Initialize Admin: Call the initialize function to set the administrator wallet address responsible for managing the tours.

Create Tour: The admin uses the create_tour function to set up a new tour (providing name, price, and total seats).

Book Tour: Users call the book_tour function, specifying the tour ID and the number of seats they wish to reserve.

Verify Records: Use the view functions (get_tour and get_booking) to check tour details and confirm successful user reservations.

Future Scope
Token Payment Integration: Process real-time payments using XLM or stablecoins (USDC) directly within the booking function.

NFT Ticketing: Mint unique, verifiable Non-Fungible Tokens (NFTs) as digital tickets for booked tours.

Dynamic Pricing System: Implement smart pricing algorithms based on demand and booking timelines.

Student Dashboard: Develop a user-friendly React.js frontend interface tailored for Hutech students to browse and book tours easily.

Review & Rating Mechanism: Allow users who have completed a tour to leave verified, on-chain reviews.

Technology Stack
Rust: Core programming language for smart contract logic.

Soroban SDK: Framework for building and testing smart contracts.

Stellar Blockchain: Infrastructure for decentralized storage, high-speed execution, and low transaction fees.

Contribution
Contributions are welcome from blockchain developers, Hutech students, and open-source enthusiasts. Fork the repository, create feature branches, and submit pull requests to help improve the platform.

License
This project is licensed under the MIT License.

Contract Detail
Contract ID: CBIWGGNDBQXDU3HKXC6PMTR5ZQWRQ2ZZFQPBB4VTL2PFCJEMJIROAYTA

Network: Stellar Testnet / Futurenet

Screenshots