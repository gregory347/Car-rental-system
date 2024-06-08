**Car Rental System**
Project Overview
The Car Rental System is a Java-based application designed to manage car rentals, customer information, and rental transactions. The system encapsulates data and behavior for a modular and maintainable structure, demonstrating Object-Oriented Programming (OOP) principles.

**Table of Contents**
Features
Class Structure
Installation
Usage
Examples
Testing
Contributing
License
Features
Add and manage cars.
Add and manage customers.
Rent and return cars.
Display available cars and registered customers.
Interactive console-based interface for user operations.
Class Structure
Car: Represents a car with attributes like license plate, model, and rental status.
Customer: Represents a customer with attributes like name and driver's license number.
RentalAgency: Manages cars and customers, handling operations like adding cars/customers, renting, and returning cars.
Main: Contains the main method to run the application, including an interactive console interface for user input.
Installation
Ensure you have Java Development Kit (JDK) installed on your system.
Clone the repository:
bash
Copy code
git clone https://github.com/gregory347/Car-rental-system/edit/main/README.md
Navigate to the project directory:
bash
Copy code
cd car-rental-system
Usage
Compile the Java files:
bash
Copy code
javac Main.java
Run the application:
bash
Copy code
java Main
Examples
Upon running the application, you will be prompted with a menu to choose various operations:

Display all cars: Lists all cars in the rental agency.
Display all customers: Lists all registered customers.
Rent a car: Prompts for customer name and car model to rent a car.
Return a car: Prompts for the car's license plate to return the rented car.
Exit: Exits the application.
Testing
The application includes several methods to ensure functionality:

Functional Tests: Verify the primary functionalities like adding cars, adding customers, renting, and returning cars.
Boundary Tests: Check scenarios with no available cars or customers not found.
User Input Tests: Validate user input and error handling.
Contributing
Contributions are welcome! Please fork the repository and create a pull request with your changes.

Fork the repository.
Create a new branch (git checkout -b feature-branch).
Make your changes and commit them (git commit -am 'Add new feature').
Push to the branch (git push origin feature-branch).
Create a new pull request.
License
This project is licensed under the MIT License. See the LICENSE file for details.
