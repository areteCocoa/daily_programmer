# Created by Thomas Ring
# August 1, 2015
# main.rb
# https://www.reddit.com/r/dailyprogrammer/comments/338p28/20150420_challenge_211_easy_the_name_game/?

def main()
      name_game("Lincoln")
end

def name_game(name)
      b = String::new(name)
      b[0] = "B"

      f = String::new(name)
      f[0] = "F"

      m = String::new(name)
      m[0] = "M"

      puts "#{name}, #{name} bo #{b},\nBonana fanna fo #{f},\nFee fy mo #{m},\n#{name}!"
end

main()
