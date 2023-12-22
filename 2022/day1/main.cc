#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

template <typename T>
void print_vec(const std::vector<T> vec)
{
    std::cout << "<";
    for (T x : vec)
    {
        std::cout << x << ", ";
    }
    std::cout << ">\n";
}

int main(int argc, char** argv)
{
    std::ifstream file("input.txt");
    std::string line;
    std::vector<std::vector<uint64_t>> food_per_elf = {{}};
    while (std::getline(file, line))
    {
        if (line.empty())
        {
            food_per_elf.push_back({});
            continue;
        }

        food_per_elf.back().push_back(std::stoll(line));
    }

    std::vector<uint64_t> calories_per_elf;
    std::transform(food_per_elf.begin(), food_per_elf.end(), std::back_inserter(calories_per_elf),
        [](const std::vector<uint64_t>& food)
        { return std::accumulate(food.begin(), food.end(), 0, std::plus<uint64_t>()); });

    uint64_t max_calories = std::accumulate(
        calories_per_elf.begin(), calories_per_elf.end(), 0, [](uint64_t res, uint64_t x) { return std::max(res, x); });

    std::sort(calories_per_elf.begin(), calories_per_elf.end(), std::greater<uint64_t>());
    std::cout << "Calories: " << calories_per_elf[0] + calories_per_elf[1] + calories_per_elf[2] << "\n";
}
