#!/bin/sh

directory="$(dirname $(dirname $0))/data/"

for file in $directory*; do
	temp=$(mktemp)

	curl 'https://www.habx.com/api/gateway/graphql' \
		-H 'content-type: application/json' \
		-H "auth-token: $TOKEN" \
		-H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36' \
		--data-raw "{\"query\":\"{lots(filters:{projectSlug:[\\\"$(jq '.meta.slug' "$file"| sed 's/^"//;s/"$//')\\\"]}paginate:{limit:-1 offset:0}){nodes{housingPlanLot{levels officialExteriorSurfaceArea officialSurfaceArea typology}program{vats{value}}externalUrlId officialSlug slug}}}\"}" \
		| jq \
			--rawfile m "$file" \
			--tab '.data.lots.nodes|map({exteriorSurfaceArea:.housingPlanLot.officialExteriorSurfaceArea,id:.slug,levels:.housingPlanLot.levels,name:.officialSlug,slug:.externalUrlId,surfaceArea:.housingPlanLot.officialSurfaceArea,typology:.housingPlanLot.typology,vat:.program.vats[]?.value})as$data|$m|fromjson|(.lots=(.lots+$data|group_by(.id)|map(.[0]+.[1])))'> $temp
	mv $temp $file
done
