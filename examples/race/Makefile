race_cpp: main.cpp
	g++ -o race_cpp main.cpp -pthread

run-cpp: race_cpp
	./race_cpp

race_cpp1: main1.cpp
	g++ -o race_cpp1 main1.cpp -pthread

run-cpp1: race_cpp1
	./race_cpp1

race_rs: main.rs
	rustc -o race_rs main.rs

run-rs: race_rs
	./race_rs

race_rs1: main1.rs
	rustc -o race_rs1 main1.rs

run-rs1: race_rs1
	./race_rs1

race_rs2: main2.rs
	rustc -o race_rs2 main2.rs

run-rs2: race_rs2
	./race_rs2
