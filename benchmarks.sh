cd contracts/benchmarks/mappers
denali-test . > ../../../tools/extract-benchmarks/bench.log
cd ../../..
cd tools/extract-benchmarks
./extract.py
cd ../..
