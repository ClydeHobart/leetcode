#include "p1276.h"

#include <vector>

using namespace std;

class Solution
{
public:
    vector<int> numOfBurgers(int tomatoSlices, int cheeseSlices)
    {
        const int allSmallTomatoSlices = 2 * cheeseSlices;

        if (tomatoSlices % 2 != 0 ||
            tomatoSlices < allSmallTomatoSlices ||
            tomatoSlices > 4 * cheeseSlices)
        {
            return {};
        }
        else
        {
            const int totalJumbo = (tomatoSlices - allSmallTomatoSlices) / 2;
            const int totalSmall = cheeseSlices - totalJumbo;

            return {totalJumbo, totalSmall};
        }
    }
};

namespace p1276
{
    void example1()
    {
        const int tomatoSlices = 16;
        const int cheeseSlices = 7;

        assert(Solution().numOfBurgers(tomatoSlices, cheeseSlices) == vector<int>{1, 6});
    }

    void example2()
    {
        const int tomatoSlices = 17;
        const int cheeseSlices = 4;

        assert(Solution().numOfBurgers(tomatoSlices, cheeseSlices) == vector<int>{});
    }

    void example3()
    {
        const int tomatoSlices = 4;
        const int cheeseSlices = 17;

        assert(Solution().numOfBurgers(tomatoSlices, cheeseSlices) == vector<int>{});
    }

    void example4()
    {
        const int tomatoSlices = 16;
        const int cheeseSlices = 8;

        assert(Solution().numOfBurgers(tomatoSlices, cheeseSlices) == vector<int>{0, 8});
    }

    void example5()
    {
        const int tomatoSlices = 16;
        const int cheeseSlices = 4;

        assert(Solution().numOfBurgers(tomatoSlices, cheeseSlices) == vector<int>{4, 0});
    }

    const test::Example examples[]{
        &example1,
        &example2,
        &example3,
        &example4,
        &example5,
        nullptr};
}