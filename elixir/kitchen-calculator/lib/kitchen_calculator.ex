defmodule KitchenCalculator do

  # -----------------------------
  # get_volume({unit, volume}) -> volume

  def get_volume({_unit, volume}),
    do: volume

  # -----------------------------
  # to_milliliter({unit, volume}) -> {:milliliter, volume}

  def to_milliliter({:cup, volume}),
    do: {:milliliter, volume * 240}

  def to_milliliter({:fluid_ounce, volume}),
    do: {:milliliter, volume * 30}

  def to_milliliter({:teaspoon, volume}),
    do: {:milliliter, volume * 5}

  def to_milliliter({:tablespoon, volume}),
    do: {:milliliter, volume * 15}

  def to_milliliter(volume_pair),
    do: {:milliliter, volume_pair |> get_volume()}

  # -----------------------------
  # from_milliliter({:milliliter, volume}, unit) -> {unit, volume}

  def from_milliliter({:milliliter, volume}, :cup = unit),
    do: {unit, volume / 240}

  def from_milliliter({:milliliter, volume}, :fluid_ounce = unit),
    do: {unit, volume / 30}

  def from_milliliter({:milliliter, volume}, :teaspoon = unit),
    do: {unit, volume / 5}

  def from_milliliter({:milliliter, volume}, :tablespoon = unit),
    do: {unit, volume / 15}

  def from_milliliter(volume_pair, :milliliter),
    do: volume_pair

  # -----------------------------
  # convert({:unit_from, volume}, unit_to) -> {unit_to, volume}

  def convert(volume_pair, unit_to),
    do: to_milliliter(volume_pair) |> from_milliliter(unit_to)
end
