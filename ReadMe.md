# SORA Space Rental System 🏢

## Description
The **SORA Space Rental System** is designed to optimize the utilization of unoccupied office spaces. This system allows host users to easily list their available offices and guest users to rent them on-demand. 🚀

This model demonstrates effective management of real estate resources and facilitates transactions between hosts and guests, reflecting an innovative approach to meeting the fluctuating needs of modern workspace environments. 🌟

### Key Objectives
- **For Hosts:** Add listings for unoccupied offices to increase their profitability. 💼
- **For Guests:** Access and rent offices according to their needs, with flexibility and without long-term commitments. 🛋️

This project not only showcases software development skills but also a deep understanding of modern business requirements in the field of real estate leasing. 💡

## Getting Started

### Prerequisites
Make sure you have the following installed:
- **Rust**: Version 1.77.2 or higher.
- **Diesel CLI**: Version 2.1.1 or higher.
- **Docker**: Docker and Docker Compose should be ready to use on your machine.

### Running the Project

To get the project running on your local machine, follow these steps:

1. **Start the Docker environment:**
   ```bash
    docker-compose up -d
    
2. **Run database migrations:**
   ```bash
    diesel migration run
    
3. **Build and run the project:**
    ```bash
    cargo build
    cargo run

4. **Accessing the Database with Adminer:**
   Adminer is set up to manage the database and can be accessed through your web browser at localhost:8081/adminer. Use this interface to manage your database directly.

## Usage

The **SORA Space Rental System** is designed to be simple and intuitive, allowing users to quickly start managing and renting office spaces.

### Creating a User

To create a user in the system:

1. Navigate to the "Inscription" section through the main menu.
2. Enter the required details such as first name, last name, and user role (Host or Guest).
3. Submit the form to register the user in the system.

### Logging In

To log in as an existing user:

1. Go to the "Connection" page.
2. Simply enter the first name and last name of your user account.
3. Press "Enter" to access the system.

### Basic Operations

Once logged in, you will be directed to the main dashboard where you can:

- **For Hosts:** Added new office spaces, view listed spaces and view your contracts.
- **For Guests:** Browse available office spaces, make rentals and view your current and past rentals.

The navigation menu provides easy access to all functionalities. The system's intuitive design ensures that you can manage or rent office spaces with minimal input and maximum efficiency.

Enjoy using the SORA Space Rental System to connect hosts and guests in the dynamic world of office rentals!

## Support and Contact Information

For support inquiries or to report issues, please follow the guidelines below to ensure your queries are addressed promptly:

- **Project Maintainer**: For urgent issues or specific inquiries related to project usage or development, please contact [Timothée Gerberon] at [timger9378@gmail.com](mailto:timger9378@gmail.com?subject=Feedback%20on%20SORA%20Space%20Rental%20System).

We welcome feedback and contributions from the community, and we’re committed to providing timely and helpful support to all our users.

