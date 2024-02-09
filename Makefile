asgn_01: ./assignment_01/main.cpp
	g++ -std=c++11 -Wall -Werror -o ./assignment_01/main ./assignment_01/main.cpp

asgn_02: ./assignment_02/main.cpp
	g++ -std=c++11 -Wall -Werror -o ./assignment_02/main ./assignment_02/main.cpp

clean:
	rm ./assignment_01/main ./assignment_02/main
