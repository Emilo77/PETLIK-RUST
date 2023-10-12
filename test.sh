#!/bin/bash

# Przechodzimy przez wszystkie pliki .in w danym folderze
for input_file in tests/*.in; do
    # Tworzymy nazwę pliku wyjściowego na podstawie pliku wejściowego
    output_file="${input_file%.in}.out"

    # Uruchamiamy Twój program z plikiem wejściowym i zapisujemy wynik w pliku temporary_output.txt
    ./petlik < "$input_file" > temporary_output.txt

    # Porównujemy wynik z oczekiwanym wyjściem
    if diff -q temporary_output.txt "$output_file" > /dev/null ; then
        echo "Test dla $input_file zakończony powodzeniem"
    else
        echo "Test dla $input_file zakończony niepowodzeniem"
        echo "Oczekiwano: $(cat $output_file)"
        echo "Otrzymano: $(cat temporary_output.txt)"
    fi

    # Usuwamy plik tymczasowy z wynikami
    rm temporary_output.txt
done
