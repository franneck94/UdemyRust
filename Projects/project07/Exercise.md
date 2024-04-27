# Prime Number Calculation with Threads Exercise

## Objective

Implement a program that calculates prime numbers using multiple threads and measures the elapsed time for the computation. This exercise aims to reinforce understanding of multithreading in Rust and synchronization using channels.

## Instructions

### Define Worker Result Struct

Create a WorkerResult struct to hold the results of prime number calculation performed by each thread.  
The struct should contain fields for the thread ID (thread_id) and a vector of prime numbers and their corresponding boolean values (results).

### Implement Worker Function

Write a worker function that represents the thread's task.  
This function should take a transmitter for sending results, the thread ID, start offset, and end offset as arguments.  
Within the function, iterate over the range specified by the offsets and calculate whether each number is prime using the is_prime function from the lib module.  
Send the results to the main thread through the transmitter.

### Implement Number Split Function

Define a number_split function that takes the thread ID as an argument and returns the start and end offsets for the range of numbers to be processed by that thread.  
Use constants for the start value, range size, and calculate the offsets accordingly.

### Main Function

In the main function, initialize a vector to hold thread handles. Start measuring the elapsed time using Instantnow().  
Create a channel for communication between threads. Spawn multiple worker threads, each with its own ID and range of numbers to process.  
Collect the thread handles in the vector.

### Receive and Process Results

Receive and process the results from each worker thread using the receiver iterator.  
Print out the prime numbers and their corresponding thread IDs.  
Join all the worker threads to ensure they finish execution.

### Measure Elapsed Time

Calculate the elapsed time by subtracting the start time from the end time.   
Print out the elapsed time at the end of the program.

#### Example Output

```
0 - Thread 0, 500000027 is prime true
1 - Thread 1, 500000007 is prime true
2 - Thread 2, 500000009 is prime true
3 - Thread 3, 500000011 is prime true
4 - Thread 4, 500000029 is prime true
5 - Thread 5, 500000051 is prime true
6 - Thread 6, 500000003 is prime true
7 - Thread 7, 500000017 is prime true
8 - Thread 8, 500000021 is prime true
9 - Thread 9, 500000023 is prime true
Elapsed Time 0.504s
```

Happy coding and multithreading!
