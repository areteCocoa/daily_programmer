-module(main).

-compile(export_all).

%smallest_base(A) ->
    % testing

largest_digit(Number) ->
    largest_digit(string_from_number(Number), 0).

largest_digit([H|T], 0) ->
    largest_digit(T, H);
largest_digit([H|T], Largest) ->
    L = if H > Largest -> H;
    		 true -> Largest
    	  end,
   largest_digit(T, L);
largest_digit([], Largest)  ->
   Largest.


string_from_number(A) ->
    lists:flatten(io_lib:format("~p", [A])).
