echo " "
echo " "
echo "+----------------------+"
echo "|      signature       |"
echo "+----------------------+"
echo " "
./surrealism-runtime sig --name can_drive out.wasm

echo " "
echo " "
echo "+----------------------+"
echo "|        invoke        |"
echo "+----------------------+"
echo " "
./surrealism-runtime run --name can_drive --arg 18 out.wasm
echo " "
./surrealism-runtime run --name can_drive --arg 17 out.wasm