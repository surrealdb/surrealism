echo "Information for 'demo.surli':"
./surrealism info demo.surli

echo ""
echo "Signature for 'can_drive' in 'demo.surli':"
./surrealism sig --fnc can_drive demo.surli

echo ""
echo "Running 'can_drive' function with argument 17 in 'demo.surli':"
./surrealism run --fnc can_drive --arg 17 demo.surli

echo ""
echo "Running 'can_drive' function with argument 18 in 'demo.surli':"
./surrealism run --fnc can_drive --arg 18 demo.surli