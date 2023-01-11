#!/bin/sh

directory="$(dirname $(dirname $0))/data/"

manifests=$(
	curl 'https://www.habx.com/api/gateway/graphql' \
		-H 'content-type: application/json' \
		-H "auth-token: $TOKEN" \
		-H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36' \
		--data-raw '{"query":"{prj{projectProperties(filters:{types:[threeD]}paginate:{limit:-1 offset:0}){nodes{value}}}}"}' \
		| jq '.data.prj.projectProperties.nodes[].value.orbitalUrl' \
		| grep -vE 'parcours-2020-demo|test-orbitale-training' \
		| sort | uniq -u \
		| sed 's/^"//;s/"$//;s/\/$/\/orbit.json/' \
)

for manifest in $manifests; do
	curl $manifest \
		| jq \
			--arg p "$(echo $manifest| sed 's/\/[^\/]*$/\//')" \
			--arg s "$(echo $manifest| sed 's/^.*\/projects-3d\/\([^\/]*\)\/.*$/\1/')" \
			--tab '{meta:(.meta+{path:$p,slug:$s}),lots,views}' \
		> "$directory$(echo $manifest| sed 's/^.*\/projects-3d\/\([^\/]*\)\/.*$/\1/').json"
done
