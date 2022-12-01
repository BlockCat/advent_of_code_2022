import requests
import os
from dotenv import load_dotenv
from os.path import exists

load_dotenv()

print('Enter day:')
year = 2022
day = input()
url = "https://adventofcode.com/{year}/day/{day}/input".format(year = year, day = day)

example_path = 'examples/day_{}.rs'.format(day.zfill(2))
input_path = 'input/day_{}.txt'.format(day.zfill(2))

if not exists(example_path):
    print('created: ' + example_path)
    file = open(example_path, 'x')
    rust = '''pub fn main() {
    let input_text = include_str!("../input/day_{day}.txt");

    // println!("Exercise 1: {}", exercise_1(&numbers));
    // println!("Exercise 2: {}", exercise_2(&numbers));
}

fn exercise_1(input: usize) -> usize {
    unimplemented!()
}
fn exercise_2(input: usize) -> usize {
    unimplemented!()    
}'''.replace('{day}', day.zfill(2))
    file.write(rust)
    file.close()


if not exists(input_path):
    print('created:' + input_path)
    file = open(input_path, 'x')
    cookies = {'session': os.environ["COOKIE"]}
    headers = {'User-Agent': 'https://github.com/BlockCat/advent_of_code_2022 by BlockCat'}
    response = requests.get(url = url, cookies=cookies, headers=headers)
    file.write(response.text)
    file.close()