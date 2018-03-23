all:
	cargo run --release src/demo.jaz
	mv src/out.cpp src/demo.cpp 
	echo "OUTPUT (src/demo.cpp):\n"
	g++ src/demo.cpp -o out && ./out
	cargo run --release src/foo.jaz
	mv src/out.cpp src/foo.cpp
	echo "OUTPUT (src/foo.cpp):\n"
	g++ src/foo.cpp -o out && ./out
	cargo run --release src/operatorsTest.jaz
	mv src/out.cpp src/operatorsTest.cpp
	echo "OUTPUT (src/operatorsTest.cpp):\n"
	g++ src/operatorsTest.cpp -o out && ./out
	cargo run --release src/factProc.jaz 
	mv src/out.cpp src/factProc.cpp 
	echo "OUTPUT (src/factProc.cpp):\n"
	g++ src/factProc.cpp -o out && ./out
