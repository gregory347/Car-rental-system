import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

class Car {
    private String licensePlate;
    private String model;
    private boolean isRented;

    public Car(String licensePlate, String model) {
        this.licensePlate = licensePlate;
        this.model = model;
        this.isRented = false;
    }

    public String getLicensePlate() {
        return licensePlate;
    }

    public String getModel() {
        return model;
    }

    public boolean isRented() {
        return isRented;
    }

    public void rentCar() {
        this.isRented = true;
    }

    public void returnCar() {
        this.isRented = false;
    }

    @Override
    public String toString() {
        return "Car [licensePlate=" + licensePlate + ", model=" + model + ", isRented=" + isRented + "]";
    }
}

class Customer {
    private String name;
    private String driverLicenseNumber;

    public Customer(String name, String driverLicenseNumber) {
        this.name = name;
        this.driverLicenseNumber = driverLicenseNumber;
    }

    public String getName() {
        return name;
    }

    public String getDriverLicenseNumber() {
        return driverLicenseNumber;
    }

    @Override
    public String toString() {
        return "Customer [name=" + name + ", driverLicenseNumber=" + driverLicenseNumber + "]";
    }
}

class RentalAgency {
    List<Car> cars;
    List<Customer> customers;

    public RentalAgency() {
        this.cars = new ArrayList<>();
        this.customers = new ArrayList<>();
    }

    public void addCar(Car car) {
        cars.add(car);
    }

    public void addCustomer(Customer customer) {
        customers.add(customer);
    }

    public Car findAvailableCarByModel(String model) {
        for (Car car : cars) {
            if (car.getModel().equalsIgnoreCase(model) && !car.isRented()) {
                return car;
            }
        }
        return null;
    }

    public boolean rentCar(Customer customer, String model) {
        Car availableCar = findAvailableCarByModel(model);
        if (availableCar != null) {
            availableCar.rentCar();
            System.out.println("Car rented to " + customer.getName() + ": " + availableCar);
            return true;
        } else {
            System.out.println("No available cars of model " + model + " for " + customer.getName());
            return false;
        }
    }

    public void returnCar(Car car) {
        car.returnCar();
        System.out.println("Car returned: " + car);
    }

    public void displayCars() {
        for (Car car : cars) {
            System.out.println(car);
        }
    }

    public void displayCustomers() {
        for (Customer customer : customers) {
            System.out.println(customer);
        }
    }
}

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        RentalAgency agency = new RentalAgency();

        Car car1 = new Car("KDA 400Z", "SUBARU LEGACY");
        Car car2 = new Car("KDD 113K", "SUBARU FORESTER XT");
        agency.addCar(car1);
        agency.addCar(car2);

        Customer customer1 = new Customer("kipngeno", "D1234567");
        Customer customer2 = new Customer("kiprotich", "D7654321");
        agency.addCustomer(customer1);
        agency.addCustomer(customer2);

        System.out.println("Cars available:");
        agency.displayCars();
        System.out.println("\nCustomers:");
        agency.displayCustomers();

        System.out.print("\nEnter the customer name to rent a car: ");
        String customerName = scanner.nextLine();

        Customer rentingCustomer = null;
        for (Customer customer : agency.customers) {
            if (customer.getName().equalsIgnoreCase(customerName)) {
                rentingCustomer = customer;
                break;
            }
        }

        if (rentingCustomer != null) {
            System.out.print("Enter the car model to rent: ");
            String carModel = scanner.nextLine();

            agency.rentCar(rentingCustomer, carModel);

            System.out.println("\nCars available after rental:");
            agency.displayCars();
        } else {
            System.out.println("Customer not found.");
        }

        System.out.print("\nEnter the license plate of the car to return: ");
        String licensePlate = scanner.nextLine();

        Car returningCar = null;
        for (Car car : agency.cars) {
            if (car.getLicensePlate().equalsIgnoreCase(licensePlate)) {
                returningCar = car;
                break;
            }
        }

        if (returningCar != null) {
            agency.returnCar(returningCar);

            System.out.println("\nCars available after return:");
            agency.displayCars();
        } else {
            System.out.println("Car not found.");
        }

        scanner.close();
    }
}
