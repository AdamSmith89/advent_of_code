#!/bin/bash

# Input e.g. year2023::day01
YEAR_DAY=(${1//::/ })
YEAR=${YEAR_DAY[0]}
DAY=${YEAR_DAY[1]}

#=====Project Files=====
echo -n "Setting up puzzle files..."
mkdir ./src/$YEAR 2>/dev/null
cp ./.aoc/template.rs ./src/$YEAR/$DAY.rs

sed -i "/\/\/ NEXT/i puzzle!($YEAR, $DAY)," ./src/main.rs
rustfmt ./src/main.rs

sed -i "/\/\/ NEXT/i pub mod $DAY;" ./src/lib.rs
rustfmt ./src/lib.rs
echo ✔️

echo -n "Setting up test files..."
mkdir ./tests/$YEAR 2>/dev/null
cp ./.aoc/test_template.rs ./tests/$YEAR/${DAY}_test.rs
sed -i "s/YYYY/$YEAR/g" ./tests/$YEAR/${DAY}_test.rs
sed -i "s/DD/$DAY/g" ./tests/$YEAR/${DAY}_test.rs

sed -i "/\/\/ NEXT/i mod ${DAY}_test;" ./tests/test.rs
rustfmt ./tests/test.rs
echo ✔️

#======Input File======
echo -n "Setting up input file..."
if [ -e ./input/$YEAR/$DAY.txt ]
then
    echo ✔️
else    
    SESSION=$(<$2)
    
    YEAR_NUM=(${YEAR//year/ })
    DAY_NUM=(${DAY//day/ })
    DAY_NUM_STRIPPED=${DAY_NUM#0}    
    URL=https://adventofcode.com/$YEAR_NUM/day/$DAY_NUM_STRIPPED/input

    echo -n ⬇️
    response=$(curl -s -w "%{http_code}" $URL --cookie "session=$SESSION")
    http_code=$(tail -n1 <<< "$response")  # get the last line
    
    if [[ $http_code == 200 ]]
    then
        content=$(sed '$ d' <<< "$response")   # get all but the last line which contains the status code
        mkdir ./input/$YEAR 2>/dev/null
        echo "$content" > ./input/$YEAR/$DAY.txt
        echo ✔️
    else
        echo "❌ ($http_code)"
    fi
fi

echo
echo "Happy puzzling! 🎅"