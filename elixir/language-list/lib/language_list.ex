defmodule LanguageList do
  def new(),
    do: []

  def add(list, language),
    do: [ language | list ]

  def remove([ _ | tail ]),
    do: tail

  def first([ head | _ ]),
    do: head 

  def count(list),
    do: list |> length

  def functional_list?(list),
    do: "Elixir" in list
end
