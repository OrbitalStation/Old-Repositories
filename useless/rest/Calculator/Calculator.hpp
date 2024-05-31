#ifndef _MPL_CALCULATOR_HPP
#define _MPL_CALCULATOR_HPP

#include <string>
#include <cmath>
#include <algorithm>

/*
 *
 * (, ) - 7
 * +(unary), -(unary) - 6
 * ~ - 5
 * ** - 4
 * %, *, / - 3
 * +(binary), -(binary) - 2
 * &, ^, | - 1
 *
 */

namespace mpl {

    class Calculator {

    private:

        enum class Tokens {

            Plus,
            Minus,

            Multiple,
            Pow,
            Divide,
            PercentDivide,

            Bitand,
            Bitor,
            Bitxor,
            Bitnot,

            Number,

            EOE, /* End Of Expression */
            ErrorToken = -1

        };

        struct Token {
            Tokens type;
            double value;

            Token() : type(Tokens::ErrorToken), value(0.) { }
            explicit Token(const Tokens &type, const double &value = 0.) : type(type), value(value) { }
        };

        static int to_digit(const char &c) { return isdigit(c) ? int(c) - 48 : -1; }

        static Token get_next_token(std::string &string, const bool &isOperand) {

            if (string.empty()) return Token(Tokens::EOE);

            char symbol = string[0];
            string.erase(string.begin());

            bool isFoundPeriod = false;
            bool isNegative = false;

            if (symbol == '-') {
                if (!isOperand || string.empty()) return Token(Tokens::Minus, '-');

                isNegative = true;

                symbol = string[0];
                string.erase(string.begin());
            }

            if (symbol == '+') return Token(Tokens::Plus, '+');
            if (symbol == '/') return Token(Tokens::Divide, '/');
            if (symbol == '%') return Token(Tokens::PercentDivide, '%');
            if (symbol == '&') return Token(Tokens::Bitand, '&');
            if (symbol == '|') return Token(Tokens::Bitor, '|');
            if (symbol == '^') return Token(Tokens::Bitxor, '^');
            if (symbol == '~') return Token(Tokens::Bitnot, '~');

            if (symbol == '*') {
                if (string.empty()) return Token(Tokens::Multiple, '*');

                if (string[0] != '*') return Token(Tokens::Multiple, '*');
                string.erase(string.begin());

                return Token(Tokens::Pow);

            }

            if (isdigit(symbol) || symbol == '.') {

                if (symbol == '.') isFoundPeriod = true;

                if (string.empty()) return Token(Tokens::Number, to_digit(symbol));

                std::string value;
                value.push_back(symbol);

                if(!isdigit(string[0]) && (string[0] != '.' && !isFoundPeriod)) return Token(Tokens::Number, to_digit(symbol) * (isNegative ? -1 : 1));
                symbol = string[0];
                string.erase(string.begin());
                value.push_back(symbol);

                for (unsigned long int i = 0, len = string.length(); i < len; ++i) {
                    symbol = string[0];

                    if (!isdigit(symbol) && symbol != '.') break;

                    value.push_back(symbol);
                    string.erase(string.begin());
                }

                char *end;
                return Token(Tokens::Number, strtod(value.c_str(), &end)  * (isNegative ? -1 : 1));

            }

            return Token(Tokens::ErrorToken);

        }

        static void erase_token(std::string &string) {

            if (string.empty()) return;

            char s;

            while (true) {

                string.erase(string.end() - 1);
                if (string.empty()) return;
                s = string[string.length() - 1];

                if (s == '/' || s == '%' || s == '*' || s == '&' || s == '|' || s == '^') return;
                if (s == '-') {
                    string.erase(string.end() - 1);

                    if (string.empty()) return;
                    s = string[string.length() - 1];

                    if (s != '-') {
                        string.push_back('-');
                        return;
                    }
                }

                if (s == '+') {
                    string.erase(string.end() - 1);

                    if (string.empty()) return;
                    s = string[string.length() - 1];

                    if (s != '+') {
                        string.push_back('+');
                        return;
                    }

                }

            }

        }

        static void compress(std::string &string) {
            std::string temp;
            for (auto iterator = string.begin(); iterator != string.end(); ++iterator)
                if (*iterator == ' ' && (!isdigit(*(iterator - 1)) || !isdigit(*(iterator + 1))))
                    string.erase(iterator--);
        }

        static bool isOperator(const Tokens &type) {
            return (type == Tokens::Plus || type == Tokens::Minus ||
                type == Tokens::Multiple || type == Tokens::Divide || type == Tokens::PercentDivide ||
                type == Tokens::Bitand || type == Tokens::Bitor || type == Tokens::Bitxor || type == Tokens::Bitnot ||
                type == Tokens::Pow);
        }

        static void calculate_all_brackets(std::string &string) {
            std::string str;

            for (unsigned long int i = 0, i2, n; i < string.length();) {

                if (string[i] == '(') {
                    n = 1;
                    i2 = i++;

                    do {

                        if (i >= string.length()) throw bad_syntax();
                        if (string[i] == '(') ++n;
                        else if (string[i] == ')') --n;

                        if (n == 0) break;

                        str += string[i++];

                    } while (true);

                    string.erase(i2, i - i2 + 1);
                    string.insert(i2, std::to_string(_calculate(str)));

                } else ++i;

            }

        }

        static void calculate_all_unary_pluses_and_minuses_helper(std::string &str2, std::string &str3, Token &valT, double &value, const bool &isRToV) {

            test:

            valT = get_next_token(str2, false);

            if (valT.type == Tokens::Minus) {
                value = -value;
                goto test;
            } else if (valT.type == Tokens::Plus) {
                goto test;
            } else if (valT.type == Tokens::Number) {
                if (isRToV) value *= valT.value;
            } else if (valT.type == Tokens::Bitnot) {
                str3 += '~';
                goto test;
            } else throw bad_syntax();

        }

        static void calculate_all_unary_pluses_and_minuses(std::string &string) {

            Token valT, opT;
            double value;
            std::string str2 = string, str3, temp;

            while (!str2.empty()) {

                value = 1.;

                calculate_all_unary_pluses_and_minuses_helper(str2, str3, valT, value, true);

                if (str2.empty()) {
                    //erase_token(str3);
                    str3 += std::to_string(value);
                    break;
                }

                opT = get_next_token(str2, false);

                calculate_all_unary_pluses_and_minuses_helper(str2, str3, valT, value, false);

                erase_token(str3);
                temp = std::to_string(valT.value);
                str3 += std::to_string(value);

                if (opT.type == Tokens::Pow) str3 += "**";
                else str3 += char(opT.value);

                str3 += temp;

                if (str2.empty()) break;
                str2.insert(0, temp);

            }

            string = str3;

        }

        static void calculate_all_bitnots_helper(Token &valT, std::string &str2) {

            bool make = false;

            test:

            valT = get_next_token(str2, true);

            if (valT.type == Tokens::Bitnot) {
                make = !make;
                goto test;
            } else if (valT.type == Tokens::Number) {
                if (make) {
                    int temp = int(valT.value);
                    if (valT.value != double(temp)) throw bad_syntax();
                    valT.value = ~temp;
                }
            }

        }

        static void calculate_all_bitnots(std::string &string) {

            Token valT, opT;
            double value;
            std::string str2 = string, str3, temp;

            while (!str2.empty()) {

                calculate_all_bitnots_helper(valT, str2);

                value = valT.value;

                if (str2.empty()) {
                    str3 += std::to_string(value);
                    break;
                }

                opT = get_next_token(str2, false);

                calculate_all_bitnots_helper(valT, str2);

                erase_token(str3);
                temp = std::to_string(valT.value);
                str3 += std::to_string(value);

                if (opT.type == Tokens::Pow) str3 += "**";
                else str3 += char(opT.value);

                str3 += temp;

                if (str2.empty()) break;
                str2.insert(0, temp);

            }

            string = str3;

        }

        static void calculate_all_powers(std::string &string) {

            std::string str2 = string, str3, temp;
            Token opT, valT;
            double value;

            do {

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (str2.empty()) {
                    if (str3.empty()) return;
                    break;
                }

                value = valT.value;

                opT = get_next_token(str2, false);
                if (!isOperator(opT.type)) throw bad_syntax();

                valT = get_next_token(str2, true);
                if (valT.type != Tokens::Number) throw bad_syntax();

                if (opT.type == Tokens::Pow) {

                    temp = std::to_string(pow(value,valT.value));
                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;
                    str2.insert(0, temp);

                } else {

                    temp = std::to_string(value) + char(opT.value) + std::to_string(valT.value);
                    erase_token(str3);
                    str3 += temp;

                    str2.insert(0, std::to_string(valT.value));

                }

            } while (true);

            string = str3;

        }

        static void calculate_all_multiplies_and_divides(std::string &string) {

            std::string str2 = string, str3, temp;

            Token opT, valT;

            double value;

            int temp1, temp2;

            do {

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (str2.empty()) {
                    if (str3.empty()) return;
                    break;
                }

                value = valT.value;

                opT = get_next_token(str2, false);

                if (!isOperator(opT.type)) throw bad_syntax();

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (opT.type == Tokens::Multiple) {

                    temp = std::to_string(value * valT.value);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else if (opT.type == Tokens::Divide) {

                    temp = std::to_string(value / valT.value);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else if (opT.type == Tokens::PercentDivide) {

                    temp1 = int(value);

                    if (double(temp1) != value) throw bad_syntax();

                    temp2 = int(valT.value);

                    if (double(temp2) != valT.value) throw bad_syntax();

                    temp = std::to_string(temp1 % temp2);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else {

                    temp = std::to_string(value) + char(opT.value) + std::to_string(valT.value);

                    erase_token(str3);

                    str3 += temp;

                    str2.insert(0, std::to_string(valT.value));

                }

            } while (true);

            string = str3;

        }

        static void calculate_all_pluses_and_minuses(std::string &string) {

            std::string str2 = string, str3, temp;

            Token opT, valT;

            double value;

            do {

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (str2.empty()) {
                    if (str3.empty()) return;
                    break;
                }

                value = valT.value;

                opT = get_next_token(str2, false);

                if (!isOperator(opT.type)) throw bad_syntax();

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (opT.type == Tokens::Plus) {

                    temp = std::to_string(value + valT.value);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else if (opT.type == Tokens::Minus) {

                    temp = std::to_string(value - valT.value);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else {

                    temp = std::to_string(value) + char(opT.value) + std::to_string(valT.value);

                    erase_token(str3);

                    str3 += temp;

                    str2.insert(0, std::to_string(valT.value));

                }

            } while (true);

            string = str3;

        }

        static void calculate_all_bit_operations(std::string &string) {

            std::string str2 = string, str3, temp;

            Token opT, valT;

            double value;

            int temp1, temp2;

            do {

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (str2.empty()) {
                    if (str3.empty()) return;
                    break;
                }

                value = valT.value;

                opT = get_next_token(str2, false);

                if (!isOperator(opT.type)) throw bad_syntax();

                valT = get_next_token(str2, true);

                if (valT.type != Tokens::Number) throw bad_syntax();

                if (opT.type == Tokens::Bitand) {

                    temp1 = int(value);

                    if (double(temp1) != value) throw bad_syntax();

                    temp2 = int(valT.value);

                    if (double(temp2) != valT.value) throw bad_syntax();

                    temp = std::to_string(temp1 & temp2);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else if (opT.type == Tokens::Bitor) {

                    temp1 = int(value);

                    if (double(temp1) != value) throw bad_syntax();

                    temp2 = int(valT.value);

                    if (double(temp2) != valT.value) throw bad_syntax();

                    temp = std::to_string(temp1 | temp2);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else if (opT.type == Tokens::Bitxor) {

                    temp1 = int(value);

                    if (double(temp1) != value) throw bad_syntax();

                    temp2 = int(valT.value);

                    if (double(temp2) != valT.value) throw bad_syntax();

                    temp = std::to_string(temp1 ^ temp2);

                    erase_token(str3);

                    str3 += temp;

                    if (str2.empty()) break;

                    str2.insert(0, temp);

                } else {

                    temp = std::to_string(value) + char(opT.value) + std::to_string(valT.value);

                    erase_token(str3);

                    str3 += temp;

                    str2.insert(0, std::to_string(valT.value));

                }

            } while (true);

            string = str3;

        }

        static double _calculate(std::string &string) {

            calculate_all_brackets(string);

            calculate_all_unary_pluses_and_minuses(string);

            calculate_all_bitnots(string);

            calculate_all_powers(string);

            calculate_all_multiplies_and_divides(string);

            calculate_all_pluses_and_minuses(string);

            calculate_all_bit_operations(string);

            char *end;

            return strtod(string.c_str(), &end);

        }

    public:

        struct bad_ptr : std::exception { };

        struct bad_syntax : std::exception { };

        static double calculate(const char * const &string) {

            if (string == nullptr) throw bad_ptr();

            std::string str = string;

            compress(str);

            return _calculate(str);

        }


    };

}

#endif /* _MPL_CALCULATOR_HPP */
