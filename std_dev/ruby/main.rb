# Created by Thomas Ring
# August 1, 2015
# main.rb
# https://www.reddit.com/r/dailyprogrammer/comments/35l5eo/20150511_challenge_214_easy_calculating_the/?

def std_dev(numbers)
      sum = 0
      for number in numbers
            sum += number
      end
      mean = sum / numbers.length

      variance_sum = 0
      for number in numbers
            variance_sum += (number - mean)**2
      end
      variance = variance_sum / numbers.length

      return Math::sqrt(variance)
end

n = [5, 6, 11, 13, 19, 20, 25, 26, 28, 37]
puts std_dev(n)

n2 = [37, 81, 86, 91, 97, 108, 109, 112, 112, 114, 115, 117, 121, 123, 141]
puts std_dev(n2)
