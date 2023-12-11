# Input e.g. year2023::day01
YEAR_DAY=(${1//::/ })
YEAR=${YEAR_DAY[0]}
DAY=${YEAR_DAY[1]}

echo "=====Project Files====="
echo -n "Setting up puzzle files..."
mkdir ./src/$YEAR 2>/dev/null
cp ./util/template.rs ./src/$YEAR/$DAY.rs

sed -i "/\/\/ NEXT/i puzzle!($YEAR, $DAY)," ./src/main.rs
rustfmt ./src/main.rs

sed -i "/\/\/ NEXT/i pub mod $DAY;" ./src/lib.rs
rustfmt ./src/lib.rs
echo ✔️

echo -n "Setting up test files..."
mkdir ./tests/$YEAR 2>/dev/null
cp ./util/test_template.rs ./tests/$YEAR/${DAY}_test.rs
sed -i "s/YYYY/$YEAR/g" ./tests/$YEAR/${DAY}_test.rs
sed -i "s/DD/$DAY/g" ./tests/$YEAR/${DAY}_test.rs

sed -i "/\/\/ NEXT/i mod ${DAY}_test;" ./tests/test.rs
rustfmt ./tests/test.rs
echo ✔️
echo

echo "======Input File======"
echo -n "Setting up input file..."
if [ -e ./input/$YEAR/$DAY.txt ]
then
    echo ✔️
else
    mkdir ./input/$YEAR 2>/dev/null
    touch ./input/$YEAR/$DAY.txt
    
    SESSION=$(<$2)
    
    YEAR_NUM=(${YEAR//year/ })
    DAY_NUM=(${DAY//day/ })
    
    echo -n ⬇️
    curl -o ./input/$YEAR/$DAY.txt https://adventofcode.com/$YEAR_NUM/day/$DAY_NUM/input --cookie "session=$SESSION" 2>/dev/null
    echo ✔️
fi

echo
echo "Happy puzzling! 🎅"