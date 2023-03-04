#include <algorithm>
#include <vector>

#include "test.h"

#define PROBLEM_34_INCLUDES

namespace p34
{
    using namespace std;

#include <algorithm>
#include <vector>

    class Solution
    {
    public:
        vector<int> searchRange(vector<int> &nums, int target)
        {
            int start, end;

            findStartAndEnd(nums, start, end, target);

            return {start, end};
        }

        static void findStartAndEnd(vector<int> &nums, int &outStart, int &outEnd, int target)
        {
            outStart = -1;
            outEnd = -1;

            if (nums.size() == 0)
            {
                return;
            }

            const int last = nums.size() - 1;
            const auto compareToBoundary = [&, target, last](int index, bool isStart) -> int
            {
                const int compareToTarget = nums[index] - target;

                if (compareToTarget != 0)
                {
                    return compareToTarget;
                }

                if (isStart)
                {
                    if (index == 0 || nums[index - 1] < target)
                    {
                        return 0;
                    }
                    else
                    {
                        return 1;
                    }
                }
                else
                {
                    if (index == last || nums[index + 1] > target)
                    {
                        return 0;
                    }
                    else
                    {
                        return -1;
                    }
                }
            };
            const auto tryFindBoundary = [&, last](int &boundary, int left, int right, bool isStart) -> bool
            {
                int middle;
                int compareMiddleToBoundary;

                while (true)
                {
                    middle = (right + left) / 2;
                    compareMiddleToBoundary = compareToBoundary(middle, isStart);

                    if (compareMiddleToBoundary == 0)
                    {
                        boundary = middle;

                        return true;
                    }
                    else if (left == right)
                    {
                        return false;
                    }
                    else if (compareMiddleToBoundary < 0)
                    {
                        left = min(middle + 1, right);
                    }
                    else
                    {
                        right = max(left, middle - 1);
                    }
                }
            };

            int start, end;

            if (tryFindBoundary(start, 0, last, true) && tryFindBoundary(end, start, last, false))
            {
                outStart = start;
                outEnd = end;
            }
        }
    };

    void example1()
    {
        vector<int> nums{5, 7, 7, 8, 8, 10};
        const int target = 8;

        assert(Solution().searchRange(nums, target) == vector<int>{3, 4});
    }

    void example2()
    {
        vector<int> nums{5, 7, 7, 8, 8, 10};
        const int target = 6;

        assert(Solution().searchRange(nums, target) == vector<int>{-1, -1});
    }

    void example3()
    {
        vector<int> nums{};
        const int target = 0;

        assert(Solution().searchRange(nums, target) == vector<int>{-1, -1});
    }

    static const test::Example examples[]{
        &example1,
        &example2,
        &example3};
}
