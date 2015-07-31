# Created by Thomas Ring
# July 31, 2015
# Takes a list of items and shuffles it

def main()
      list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
      puts list
      puts "SHUFFLING"
      shuffle(list)
      puts list
end

def shuffle(list)
      for index in 0..list.length-1
            # puts object
            new_index = Random.rand(list.length-1)
            old_value = list[index]
            destination_value = list[new_index]
            list[new_index] = old_value
            list[index] = destination_value
      end
end

main()
