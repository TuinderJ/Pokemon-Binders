curl "https://pokeapi.co/api/v2/pokemon?limit=2" | jq -c '.results[]' | while read -r pokemons; do
  url=$(echo "$pokemons" | jq -r '.url')
  curl "$url" | jq -c '{id: .id, name: .name, sprite: .sprites.front_default}' | while read -r pokemon; do
    name=$(echo "$pokemon" | jq -r '.name')
    id=$(echo "$pokemon" | jq -r '.id')
    sprite=$(echo "$pokemon" | jq -r '.sprite')
    echo "$name, $id, $sprite"
  done
done

