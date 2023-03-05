#include "p420.h"

// #define ZB_DEBUG

#include <algorithm>
#include <cctype>
// #include <iomanip>
// #include <iostream>
#include <string>

using namespace std;

#ifdef ZB_DEBUG
#define ZB_DEBUG_ONLY(...) __VA_ARGS__
#else // ZB_DEBUG
#define ZB_DEBUG_ONLY(...)
#endif // ZB_DEBUG

class Solution
{
private:
    static constexpr int MIN_SIZE = 6;
    static constexpr int MAX_SIZE = 20;
    static constexpr char LOWERCASE_BIT = 0x20;
    static constexpr char UPPERCASE_MASK = ~LOWERCASE_BIT;

public:
    int strongPasswordChecker(string password)
    {
        this->password = move(password);
        init();

        // cout << "0 ( 0): \"" << this->password << "\"\n";
        fixRule3();
        // cout << "1 (" << setw(2) << (int)steps << "): \"" << this->password << "\"\n";
        fixRule2();
        // cout << "2 (" << setw(2) << (int)steps << "): \"" << this->password << "\"\n";
        fixRule1();
        // cout << "3 (" << setw(2) << (int)steps << "): \"" << this->password << "\"\n";

        return steps;
    }

private:
    void init()
    {
        password.reserve(MIN_SIZE);

        char lowercasesVal = 0;
        char uppercasesVal = 0;
        char digitsVal = 0;

        for (const char c : password)
        {
            if (islower(c))
            {
                ++lowercasesVal;

#ifndef ZB_DEBUG
                if (uppercasesVal && digitsVal)
                {
                    break;
                }
#endif // ZB_DEBUG
            }
            else if (isupper(c))
            {
                ++uppercasesVal;

#ifndef ZB_DEBUG
                if (lowercasesVal && digitsVal)
                {
                    break;
                }
#endif // ZB_DEBUG
            }
            else if (isdigit(c))
            {
                ++digitsVal;

#ifndef ZB_DEBUG
                if (lowercasesVal && uppercasesVal)
                {
                    break;
                }
#endif // ZB_DEBUG
            }
        }

        lowercases = lowercasesVal;
        uppercases = uppercasesVal;
        digits = digitsVal;
        steps = 0;
    }

    void fixRule1()
    {
        if (isTooShort())
        {
#ifdef ZB_DEBUG
            if (password.empty())
            {
                password.push_back('a');
                ZB_DEBUG_ONLY(++lowercases);
                ++steps;
            }

            do
            {
                password.push_back(getNewChar(password.back(), false));
                ++steps;
            } while (isTooShort());
#else  // ZB_DEBUG
            steps += static_cast<char>(MIN_SIZE - password.size());
#endif // ZB_DEBUG
        }
        else if (isTooLong())
        {
            steps += static_cast<char>(password.size() - MAX_SIZE);

#ifdef ZB_DEBUG
            int firstLowercase = -1, firstUppercase = -1, firstDigit = -1;
            bool foundLowercase = false, foundUppercase = false, foundDigit = false;

            for (int i = 0; i < password.size(); ++i)
            {
                const char c = password[i];

                if (islower(c))
                {
                    if (!foundLowercase)
                    {
                        foundLowercase = true;
                        firstLowercase = i;

                        if (foundUppercase && foundDigit)
                        {
                            break;
                        }
                    }
                }
                else if (isupper(c))
                {
                    if (!foundUppercase)
                    {
                        foundUppercase = true;
                        firstUppercase = i;

                        if (foundLowercase && foundDigit)
                        {
                            break;
                        }
                    }
                }
                else if (isdigit(c))
                {
                    if (!foundDigit)
                    {
                        foundDigit = true;
                        firstDigit = i;

                        if (foundLowercase && foundUppercase)
                        {
                            break;
                        }
                    }
                }
            }

            int firsts[3] = {firstLowercase, firstUppercase, firstDigit};

            sort(firsts, firsts + 3);

            if (firsts[2] < MAX_SIZE)
            {
                // Easy case, since we can just get rid of all that remain
                erase(MAX_SIZE, password.size());
            }
            else
            {
                static constexpr int MAX_SIZE_MINUS_1 = MAX_SIZE - 1;

                erase(firsts[2] + 1, password.size());

                if (firsts[1] < MAX_SIZE_MINUS_1)
                {
                    erase(MAX_SIZE_MINUS_1, firsts[2]);
                }
                else
                {
                    static constexpr int MAX_SIZE_MINUS_2 = MAX_SIZE - 2;

                    erase(firsts[1] + 1, firsts[2]);

                    if (firsts[0] < MAX_SIZE_MINUS_2)
                    {
                        erase(MAX_SIZE_MINUS_2, firsts[1]);
                    }
                    else
                    {
                        static constexpr int MAX_SIZE_MINUS_3 = MAX_SIZE - 3;

                        erase(firsts[0] + 1, firsts[1]);
                        erase(MAX_SIZE_MINUS_3, firsts[0]);
                    }
                }
            }
#endif // ZB_DEBUG
        }
    }

    void fixRule2()
    {
        if (isTooShort())
        {
            while (const char c = getNewChar())
            {
                password.push_back(c);
                ++steps;
            }
        }
        else
        {
            int i = 0;

            while (const char c = getNewChar())
            {
                password[i++] = c;
                ++steps;
            }
        }
    }

    void fixRule3()
    {
        struct StreakInfo
        {
            char index;
            char size;
        };

        // password is initially 50 chars at most, 50 / 3 == 16 (int division)
        StreakInfo streaks[16];
        int streaksSize = 0;

        int i = 0;

        while (i + 2 < password.size())
        {
            const char c = password[i];

            int j = i + 1;

            while (j < password.size() && password[j] == c)
            {
                ++j;
            }

            if (j - i >= 3)
            {
                streaks[streaksSize++] = {(char)i, (char)(j - i)};
            }

            i = j;
        }

        if (streaksSize == 0)
        {
            return;
        }

        // Be tactical about which streaks get shortened, since streaks with a size % 3 == 2 are
        // best to leave in for substitution if it's necessary
        while (isTooLong())
        {
            int streakToShorten = -1;

            const auto findStreakToShorten = [&](char remainder) -> bool
            {
                for (int streak = 0; streak < streaksSize; ++streak)
                {
                    const char streakSize = streaks[streak].size;

                    if (streakSize >= 3 && streakSize % 3 == remainder)
                    {
                        streakToShorten = streak;

                        break;
                    }
                }

                if (streakToShorten != -1)
                {
                    StreakInfo &streakInfo = streaks[streakToShorten];

                    ZB_DEBUG_ONLY(updateCounts(password[streakInfo.index]));
                    password.erase(streakInfo.index, 1);
                    ++steps;

                    // cout << "_ (" << setw(2) << (int)steps << "): \"" << password << "\"\n";

                    --streakInfo.size;

                    for (int streak = streakToShorten + 1; streak < streaksSize; ++streak)
                    {
                        --streaks[streak].index;
                    }

                    return true;
                }

                return false;
            };

            if (findStreakToShorten(0))
            {
                continue;
            }

            if (findStreakToShorten(1))
            {
                continue;
            }

            if (findStreakToShorten(2))
            {
                continue;
            }

            break;
        }

        for (int streak = 0; streak < streaksSize; ++streak)
        {
            StreakInfo &streakInfo = streaks[streak];

            int delta = 0;

            for (int offset = 2; offset < streakInfo.size; ++steps, offset += 3)
            {
                const int index = streakInfo.index + offset;
                const char c = password[index];
                const char newC = getNewChar(c);

                if (isTooShort())
                {
                    password.insert(index, 1, newC);
                    ++delta;
                    ++streakInfo.size;

                    // cout << "_ (" << setw(2) << (int)steps << "): \"" << password << "\"\n";
                }
                else
                {
                    ZB_DEBUG_ONLY(updateCounts(c));
                    password[index] = newC;

                    // cout << "_ (" << setw(2) << (int)steps << "): \"" << password << "\"\n";
                }
            }

            if (delta != 0)
            {
                for (int laterStreak = streak + 1; laterStreak < streaksSize; ++laterStreak)
                {
                    streaks[laterStreak].index += (char)delta;
                }
            }
        }
    }

    inline bool isTooShort() const
    {
        return password.size() < MIN_SIZE;
    }

    inline bool isTooLong() const
    {
        return password.size() > MAX_SIZE;
    }

    char getNewChar()
    {
        if (!lowercases)
        {
            lowercases = 1;

            return 'a';
        }

        if (!uppercases)
        {
            uppercases = 1;

            return 'A';
        }

        if (!digits)
        {
            digits = 1;

            return '0';
        }

        return 0;
    }

    char getNewChar(const char prev, bool skipCountChecks = false)
    {
        if (const char c = skipCountChecks ? 0 : getNewChar())
        {
            return c;
        }
        else if (isalpha(prev))
        {
            ZB_DEBUG_ONLY(++digits);

            return '0';
        }
        else
        {
            ZB_DEBUG_ONLY(++lowercases);

            return 'a';
        }
    }

    char getNewChar(const char prev, const char next)
    {
        if (const char c = getNewChar())
        {
            return c;
        }
        else if (isalpha(prev))
        {
            if (isalpha(next))
            {
                ZB_DEBUG_ONLY(++digits);

                return '0';
            }
            else if (prev & LOWERCASE_BIT)
            {
                ZB_DEBUG_ONLY(++uppercases);

                return prev & UPPERCASE_MASK;
            }
            else
            {
                ZB_DEBUG_ONLY(++lowercases);

                return prev | LOWERCASE_BIT;
            }
        }
        else
        {
            if (!isalpha(next))
            {
                ZB_DEBUG_ONLY(++lowercases);

                return 'a';
            }
            else if (next & LOWERCASE_BIT)
            {
                ZB_DEBUG_ONLY(++uppercases);

                return next & UPPERCASE_MASK;
            }
            else
            {
                ZB_DEBUG_ONLY(++lowercases);

                return next | LOWERCASE_BIT;
            }
        }
    }

#ifdef ZB_DEBUG
public:
    inline const string &getPassword() const { return password; }

private:
    void updateCounts(const char c)
    {
        if (islower(c))
        {
            --lowercases;
        }
        else if (isupper(c))
        {
            --uppercases;
        }
        else if (isdigit(c))
        {
            --digits;
        }
    }

    void erase(size_t begin_index, size_t end_index)
    {
        auto iter = password.begin() + begin_index;
        const auto end = password.begin() + end_index;

        while (iter < end)
        {
            updateCounts(*(iter++));
        }

        password.erase(begin_index, end_index - begin_index);
    }
#endif // ZB_DEBUG

private:
    string password;
    char lowercases;
    char uppercases;
    char digits;
    char steps;
};

namespace p420
{
    void example1()
    {
        Solution solution;

        assert(solution.strongPasswordChecker("a") == 5);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string("aA0a0a")));
    }

    void example2()
    {
        Solution solution;

        assert(solution.strongPasswordChecker("aA1") == 3);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string("aA1a0a")));
    }

    void example3()
    {
        assert(Solution().strongPasswordChecker("1337C0d3") == 0);
    }

    void example4()
    {
        Solution solution;

        assert(solution.strongPasswordChecker(".!.!.!.!.!.!.!.!.!.!") == 3);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string("aA0!.!.!.!.!.!.!.!.!")));
    }

    void example5()
    {
        Solution solution;

        assert(solution.strongPasswordChecker(".!.!.!.a.!.A.!.0.!.!") == 0);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string(".!.!.!.a.!.A.!.0.!.!")));
    }

    void example6()
    {
        Solution solution;

        assert(solution.strongPasswordChecker(".!.!.!.!.!.a.!.A.!.0.!.!") == 4);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string(".!.!.!.!.!.a.!.A.!.0")));
    }

    void example7()
    {
        Solution solution;

        assert(solution.strongPasswordChecker(".!.!.!.!.!.!.!.a.!.A.!.0.!.!") == 8);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string(".!.!.!.!.!.!.!.a.!A0")));
    }

    void example8()
    {
        Solution solution;

        assert(solution.strongPasswordChecker(".!.!.!.!.!.!.!.!.!.a.!.A.!.0.!.!") == 12);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string(".!.!.!.!.!.!.!.!.aA0")));
    }

    void example9()
    {
        Solution solution;

        assert(solution.strongPasswordChecker(".!.!.!.!.!.!.!.!.!.!.!.a.!.A.!.0.!.!") == 16);
        ZB_DEBUG_ONLY(assert(solution.getPassword() == string(".!.!.!.!.!.!.!.!.aA0")));
    }

    void example10()
    {
        Solution solution;

        assert(solution.strongPasswordChecker("bbaaaaaaaaaaaaaaacccccc") == 8);
    }

    void example11()
    {
        Solution solution;

        assert(solution.strongPasswordChecker("FFFFFFFFFFFFFFF11111111111111111111AAA") == 23);
    }

    void example12()
    {
        Solution solution;

        assert(solution.strongPasswordChecker("1010101010aaaB10101010"));
    }

    const test::Example examples[]{
        &example1,
        &example2,
        &example3,
        &example4,
        &example5,
        &example6,
        &example7,
        &example8,
        &example9,
        &example10,
        &example11,
        &example12,
        nullptr};
}