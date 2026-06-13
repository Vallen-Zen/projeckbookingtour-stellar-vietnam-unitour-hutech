#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, Address, Env, String};

// 1. Định nghĩa các lỗi (Errors)
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    TourNotFound = 1,
    NotEnoughSeats = 2,
    InvalidAmount = 3,
}


#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TourCounter,
    Tour(u64),
    Booking(Address, u64),
}


#[contracttype]
#[derive(Clone)]
pub struct Tour {
    pub id: u64,
    pub name: String,
    pub price: u128,        
    pub total_seats: u32,
    pub available_seats: u32,
}


#[contracttype]
#[derive(Clone)]
pub struct Booking {
    pub user: Address,
    pub tour_id: u64,
    pub seats_booked: u32,
}

#[contract]
pub struct TourBookingContract;

#[contractimpl]
impl TourBookingContract {

    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract is already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TourCounter, &0u64);
    }


    pub fn create_tour(env: Env, admin: Address, name: String, price: u128, total_seats: u32) -> u64 {

        admin.require_auth();
        

        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("Only admin can create tours");
        }


        let mut tour_id: u64 = env.storage().instance().get(&DataKey::TourCounter).unwrap();
        tour_id += 1;


        let tour = Tour {
            id: tour_id,
            name,
            price,
            total_seats,
            available_seats: total_seats,
        };

     
        env.storage().persistent().set(&DataKey::Tour(tour_id), &tour);
        env.storage().instance().set(&DataKey::TourCounter, &tour_id);

        tour_id
    }


    pub fn book_tour(env: Env, user: Address, tour_id: u64, seats_to_book: u32) -> Result<(), Error> {

        user.require_auth();

        if seats_to_book == 0 {
            return Err(Error::InvalidAmount);
        }


        let mut tour: Tour = env
            .storage()
            .persistent()
            .get(&DataKey::Tour(tour_id))
            .ok_or(Error::TourNotFound)?;


        if tour.available_seats < seats_to_book {
            return Err(Error::NotEnoughSeats);
        }

        // (Tùy chọn) Logic thanh toán token (XLM/USDC) sẽ được gọi ở đây thông qua token.transfer()

        // Cập nhật lại số lượng ghế
        tour.available_seats -= seats_to_book;
        env.storage().persistent().set(&DataKey::Tour(tour_id), &tour);


        let booking_key = DataKey::Booking(user.clone(), tour_id);
        let mut current_seats_booked = 0u32;
        if let Some(existing_booking) = env.storage().persistent().get::<_, Booking>(&booking_key) {
            current_seats_booked = existing_booking.seats_booked;
        }

        let new_booking = Booking {
            user,
            tour_id,
            seats_booked: current_seats_booked + seats_to_book,
        };

        env.storage().persistent().set(&booking_key, &new_booking);

        Ok(())
    }


    pub fn get_tour(env: Env, tour_id: u64) -> Option<Tour> {
        env.storage().persistent().get(&DataKey::Tour(tour_id))
    }


    pub fn get_booking(env: Env, user: Address, tour_id: u64) -> Option<Booking> {
        env.storage().persistent().get(&DataKey::Booking(user, tour_id))
    }
}