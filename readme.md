# 3D orbit viewer

![Preview of the viewer](public/preview.gif?raw=true)

3D orbits are nothing more than a succession of rendered images captured from a set of cameras rotating around a 3D model of a real-estate operation.
The generated images are referenced in a single JSON file, called a _manifest_. This manifest also contains additional metadata in order to represent and interact with the operation's units.

## Usage

The viewer accepts a few query parameters to personalize the user-experience:
- `manifest` (**required**) the URL to the orbit manifest of a given project
- `interactive` a boolean value enabling interactions with lots and the display of the overlay (_optional_, default to `false`)
- `redirection` a template URL used to redirect a visitor upon choosing a lot (_optional_, the feature is deactivated if omitted). Beware of special characters that might need to be encoded (see [here for details](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent)). This template supports a few placeholders, they are replaced by their corresponding value to make dynamic links:
	- `%ID%` the identifier of a lot (usually a randomly generated one, for internal usage, e.g. `83765-232`)
	- `%SLUG%` the commercial slug of a lot (most likely a combination of a lot's building, floor and index, e.g. `A201`)
- `redirection_label` the text label displayed on the lot redirection button (_optional_, defaults to `Voir les détails`)

## Availability

The viewer itself is hosted on [GitHub Pages](https://pages.github.com) at this address: https://habx.github.io/client-orbital. It should remain publicly available as long as GitHub keeps providing its services for free for open-source projects.

**The assets are temporarily stored** on [AWS S3](https://aws.amazon.com/s3) and served through [AWS CloudFront](https://aws.amazon.com/cloudfront) to help you during the transition. **You must take actions to host the data** on your cloud infrastructure, if you want to avoid an interruption of service.

## Migration

### Assets

1. Download and extract the archives along with the manifest of your project. You should receive the links from us.
2. Upload all files to your hosting solution. The files do not have to be served from the root directory. However, the assets being referenced relatively, the internal structure must remain untouched. The resources must be accessible from the domain where the viewer is hosted. Remember to review your security policies.
3. Replace the [`manifest` query parameters](#usage) of your links with the absolute URL to the `manifest.json` file.

### Viewer

The source code for the viewer is available on this repository. The project has been developed in [Rust](https://www.rust-lang.org) and is compiled down to a single [Wasm binary](https://webassembly.org/) loaded with a few other resources to an HTML file. You can download the latest version [here](https://github.com/habx/client-orbital/releases/download/1.0.0/pages.zip).

## Projects

The default links to 3D orbits (interaction enabled and redirection set when applicable) are listed below. You can edit [the query parameters](#usage) to adapt the viewer to your needs.

- [aix-les-bains-victoria](https://habx.github.io/client-orbital?manifest=data/aix-les-bains-victoria.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com/programmes-immobiliers-neufs-aix-les-bains/so--victoria-,p74771%23%ID%)
- [aix-poesia](https://habx.github.io/client-orbital?manifest=data/aix-poesia.json&interactive=true)
- [angers-arborescence](https://habx.github.io/client-orbital?manifest=data/angers-arborescence.json&interactive=true)
- [angers-cloitre-des-capucins](https://habx.github.io/client-orbital?manifest=data/angers-cloitre-des-capucins.json&interactive=true)
- [arcueil-avenue-convention](https://habx.github.io/client-orbital?manifest=data/arcueil-avenue-convention.json&interactive=true&redirection=https%3A%2F%2Farcueil-3f.fr%2Fplan%2FLOT%25SLUG%25.pdf)
- [argeles-sur-mer-castell-del-mar](https://habx.github.io/client-orbital?manifest=data/argeles-sur-mer-castell-del-mar.json)
- [argenteuil-labriere](https://habx.github.io/client-orbital?manifest=data/argenteuil-labriere.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-argenteuil%2Fvilla-nymphea%2Cp33781%23%25ID%25)
- [arssurmoselle](https://habx.github.io/client-orbital?manifest=data/arssurmoselle.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-ars-sur-moselle%2Fparenthese%2Cp98691%23%25ID%25)
- [aubervilliers-president-roosevelt](https://habx.github.io/client-orbital?manifest=data/aubervilliers-president-roosevelt.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-aubervilliers%2Fvilla-eleanor%2Cp33361%23%25ID%25)
- [bagnolet-ferry](https://habx.github.io/client-orbital?manifest=data/bagnolet-ferry.json&interactive=true)
- [bezannes-les-toits-du-golf](https://habx.github.io/client-orbital?manifest=data/bezannes-les-toits-du-golf.json&interactive=true)
- [bezons-rue-de-villeneuve](https://habx.github.io/client-orbital?manifest=data/bezons-rue-de-villeneuve.json&interactive=true&redirection=https%3A%2F%2Fwww.quanim.fr%2Fprogrammes%2Fdeuil-la-barre-le-winston%2F%23popin-program-form)
- [bischheim-urban-green](https://habx.github.io/client-orbital?manifest=data/bischheim-urban-green.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-bischheim%2Furban-green%2Cp88381%23%25ID%25)
- [bondy-edouard-vaillant](https://habx.github.io/client-orbital?manifest=data/bondy-edouard-vaillant.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-bondy%2Fles-jardins-de-beauvoir%2Cp86841%23%25ID%25)
- [bordeaux-atelier-dulamon](https://habx.github.io/client-orbital?manifest=data/bordeaux-atelier-dulamon.json&interactive=true)
- [bordeaux-brazza](https://habx.github.io/client-orbital?manifest=data/bordeaux-brazza.json&interactive=true)
- [bordeaux-eterna](https://habx.github.io/client-orbital?manifest=data/bordeaux-eterna.json&interactive=true)
- [boussy-domaine-de-la-ferme](https://habx.github.io/client-orbital?manifest=data/boussy-domaine-de-la-ferme.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-boussy-saint-antoine%2Fle-domaine-de-la-ferme%2Cp33801%23%25ID%25)
- [brianconlescimes](https://habx.github.io/client-orbital?manifest=data/brianconlescimes.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-briancon%2Fles-cimes%2Cp47231%23%25ID%25)
- [caen-detolle-cours-lavalette](https://habx.github.io/client-orbital?manifest=data/caen-detolle-cours-lavalette.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-caen%2Fcours-lavalette%2Cp72282%23%25ID%25)
- [caen-detolle](https://habx.github.io/client-orbital?manifest=data/caen-detolle.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-caen%2Fjardins-lavalette%2Cp72281%23%25ID%25)
- [cergy-boisselerie](https://habx.github.io/client-orbital?manifest=data/cergy-boisselerie.json&interactive=true)
- [champs-joliot](https://habx.github.io/client-orbital?manifest=data/champs-joliot.json&interactive=true)
- [chatenay-malabry-saphir](https://habx.github.io/client-orbital?manifest=data/chatenay-malabry-saphir.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-chatenay-malabry%2Fsaphir%2Cp67845%23%25ID%25)
- [creteil-sun-side](https://habx.github.io/client-orbital?manifest=data/creteil-sun-side.json&interactive=true)
- [deuillabarre](https://habx.github.io/client-orbital?manifest=data/deuillabarre.json&interactive=true&redirection=https%3A%2F%2Fwww.quanim.fr%2Fprogrammes%2Fdeuil-la-barre-le-winston%2F%23popin-program-form)
- [epinay-sur-orge-harmonia](https://habx.github.io/client-orbital?manifest=data/epinay-sur-orge-harmonia.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-epinay-sur-orge%2Fharmonia%2Cp30361%23%25ID%25)
- [erdeven-nationale](https://habx.github.io/client-orbital?manifest=data/erdeven-nationale.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-erdeven%2Fora%2Cp99961%23%25ID%25)
- [fabregues-le-faubourg-icade](https://habx.github.io/client-orbital?manifest=data/fabregues-le-faubourg-icade.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-fabregues%2Fle-faubourg%2Cp79121%23%25ID%25)
- [fontenay-aux-roses-villa-flora](https://habx.github.io/client-orbital?manifest=data/fontenay-aux-roses-villa-flora.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-fontenay-aux-roses%2Fvilla-flora%2Cp91171%23%25ID%25)
- [francheville-eglise](https://habx.github.io/client-orbital?manifest=data/francheville-eglise.json&interactive=true)
- [guyancourt-commedia](https://habx.github.io/client-orbital?manifest=data/guyancourt-commedia.json&interactive=true)
- [guyancourt-commedia-scene](https://habx.github.io/client-orbital?manifest=data/guyancourt-commedia-scene.json&interactive=true)
- [houilles-cityzen](https://habx.github.io/client-orbital?manifest=data/houilles-cityzen.json&interactive=true)
- [issy-les-moulineaux-29-hoche](https://habx.github.io/client-orbital?manifest=data/issy-les-moulineaux-29-hoche.json&interactive=true)
- [issy-les-moulineaux-carat](https://habx.github.io/client-orbital?manifest=data/issy-les-moulineaux-carat.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-issy-les-moulineaux%2Fcarat%2Cp89711%23%25ID%25)
- [issy-les-moulineaux-joia](https://habx.github.io/client-orbital?manifest=data/issy-les-moulineaux-joia.json&interactive=true)
- [la-crau-domaine-veraison](https://habx.github.io/client-orbital?manifest=data/la-crau-domaine-veraison.json&interactive=true&redirection=https%3A%2F%2Fdomaineveraisonlacrau.fr%2Fcontact%2F)
- [le-cannet-cinq-sens](https://habx.github.io/client-orbital?manifest=data/le-cannet-cinq-sens.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-le-cannet%2Fcinq-sens%2Cp60801%23%25ID%25)
- [le-havre-evasion](https://habx.github.io/client-orbital?manifest=data/le-havre-evasion.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-le-havre%2Fevasion%2Cp90324%3F%23%25ID%25)
- [le-plessis-robinson-agapanthe](https://habx.github.io/client-orbital?manifest=data/le-plessis-robinson-agapanthe.json&interactive=true&redirection=https%3A%2F%2Fwww.quartus-residentiel.fr%2Fappartement%2Fle-plessis-robinson-13481-%25SLUG%25)
- [les-herbiers-rue-nationale](https://habx.github.io/client-orbital?manifest=data/les-herbiers-rue-nationale.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-les-herbiers%2Fjardin-des-arts%2Cp99571%23%25ID%25)
- [liffre-les-allees-silvia](https://habx.github.io/client-orbital?manifest=data/liffre-les-allees-silvia.json&interactive=true)
- [lille-tweed](https://habx.github.io/client-orbital?manifest=data/lille-tweed.json&interactive=true)
- [limeil-brevannes-clos-de-l-orme](https://habx.github.io/client-orbital?manifest=data/limeil-brevannes-clos-de-l-orme.json&interactive=true&redirection=https%3A%2F%2Fwww.quartus-residentiel.fr%2Fappartement%2Flimeil-brevannes-19811-%25SLUG%25)
- [loos-lalcove](https://habx.github.io/client-orbital?manifest=data/loos-lalcove.json&interactive=true)
- [lyon-moselle](https://habx.github.io/client-orbital?manifest=data/lyon-moselle.json&interactive=true)
- [lyon-vertuo-part-dieu](https://habx.github.io/client-orbital?manifest=data/lyon-vertuo-part-dieu.json)
- [marcy-l-etoile-coeur-marcy](https://habx.github.io/client-orbital?manifest=data/marcy-l-etoile-coeur-marcy.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-marcy-l-etoile%2Fcoeur-marcy-%2Cp91261%23%25ID%25)
- [marseille-vogue8eme](https://habx.github.io/client-orbital?manifest=data/marseille-vogue8eme.json&interactive=true&redirection=https%3A%2F%2Fwww.quartus-residentiel.fr%2Fappartement%2Fmarseille-17921-%25SLUG%25)
- [metz-les-promenades](https://habx.github.io/client-orbital?manifest=data/metz-les-promenades.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-metz%2Fles-promenades-%2Cp99901%23%25ID%25)
- [montigny-le-bretonneux-terrasses-de-lodyssee](https://habx.github.io/client-orbital?manifest=data/montigny-le-bretonneux-terrasses-de-lodyssee.json&interactive=true&redirection=https%3A%2F%2Fwww.quanim.fr%2Fprogrammes%2Fdeuil-la-barre-le-winston%2F%23popin-program-form)
- [montpellier-sowood](https://habx.github.io/client-orbital?manifest=data/montpellier-sowood.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-montpellier%2Fso-wood%2Cp89921%23%25ID%25)
- [nancy-rose-wild](https://habx.github.io/client-orbital?manifest=data/nancy-rose-wild.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-nancy%2Frose-wild%2Cp92251%23%25ID%25)
- [nantes-be-ile](https://habx.github.io/client-orbital?manifest=data/nantes-be-ile.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-nantes%2Fbe-ile%2Cp87542%23%25ID%25)
- [nantes-boul-be-green](https://habx.github.io/client-orbital?manifest=data/nantes-boul-be-green.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-nantes%2Fbe-green%2Cp87543%3F%23%25ID%25)
- [nantes-cosmo](https://habx.github.io/client-orbital?manifest=data/nantes-cosmo.json&interactive=true)
- [nantes-joneliere](https://habx.github.io/client-orbital?manifest=data/nantes-joneliere.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-nantes%2Fvillas-erdre%2Cp93891%23%25ID%25)
- [nice-gare-du-sud-villa-rossa](https://habx.github.io/client-orbital?manifest=data/nice-gare-du-sud-villa-rossa.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-nice%2Fgare-du-sud---villa-rossa%2Cp46011%23%25ID%25)
- [noisy-le-grand-villa-verde](https://habx.github.io/client-orbital?manifest=data/noisy-le-grand-villa-verde.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-noisy-le-grand%2Fvilla-verde%2Cp80971%23%25ID%25)
- [paray-vieille-poste-villa-lumea](https://habx.github.io/client-orbital?manifest=data/paray-vieille-poste-villa-lumea.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-paray-vieille-poste%2Fvilla-lumea%2Cp40831%23%25ID%25)
- [paris13-m9a](https://habx.github.io/client-orbital?manifest=data/paris13-m9a.json&interactive=true)
- [perpignan-bella-vista](https://habx.github.io/client-orbital?manifest=data/perpignan-bella-vista.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-perpignan%2Fbella-vista-%2Cp94081%23%25ID%25)
- [pessac-le-parc-habite](https://habx.github.io/client-orbital?manifest=data/pessac-le-parc-habite.json&interactive=true)
- [plaisir-haise](https://habx.github.io/client-orbital?manifest=data/plaisir-haise.json&interactive=true&redirection=https%3A%2F%2Fresidence-pietra.fr%2Fplan%2FLOT%25SLUG%25.pdf)
- [quetigny-patio-central](https://habx.github.io/client-orbital?manifest=data/quetigny-patio-central.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-quetigny%2Fpatio-central%2Cp58431%23%25ID%25)
- [rennes-beaumont](https://habx.github.io/client-orbital?manifest=data/rennes-beaumont.json&interactive=true)
- [reze-cardinale-sud](https://habx.github.io/client-orbital?manifest=data/reze-cardinale-sud.json&interactive=true)
- [rillieuxlapape](https://habx.github.io/client-orbital?manifest=data/rillieuxlapape.json&interactive=true&redirection=https%3A%2F%2Fwww.quartus-residentiel.fr%2Fappartement%2Frillieux-la-pape-21201-%25SLUG%25)
- [romainville-gaston-roussel](https://habx.github.io/client-orbital?manifest=data/romainville-gaston-roussel.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-romainville%2Fhortea%2Cp79111%23%25ID%25)
- [roubaix-rue-racine](https://habx.github.io/client-orbital?manifest=data/roubaix-rue-racine.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-roubaix%2Fprelude%2Cp40371%23%25ID%25)
- [rueil-malmaison-l-imperiale](https://habx.github.io/client-orbital?manifest=data/rueil-malmaison-l-imperiale.json&interactive=true)
- [rueil-malmaison-sardou](https://habx.github.io/client-orbital?manifest=data/rueil-malmaison-sardou.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-rueil-malmaison%2Fvilla-tosca%2Cp32601%23%25ID%25)
- [saint-alban-leysse-bemaz](https://habx.github.io/client-orbital?manifest=data/saint-alban-leysse-bemaz.json&interactive=true)
- [saint-cloud-villa-acanthe](https://habx.github.io/client-orbital?manifest=data/saint-cloud-villa-acanthe.json&interactive=true&redirection=https%3A%2F%2Fwww.promodim.fr%2Fcontact%2F)
- [saint-cyr-accord-majeur](https://habx.github.io/client-orbital?manifest=data/saint-cyr-accord-majeur.json&interactive=true)
- [saint-genis-pouilly-le-magnifique](https://habx.github.io/client-orbital?manifest=data/saint-genis-pouilly-le-magnifique.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-saint-genis-pouilly%2Fle-magnifique%2Cp88481%23%25ID%25)
- [saint-herblain-sowood](https://habx.github.io/client-orbital?manifest=data/saint-herblain-sowood.json&interactive=true&redirection=https%3A%2F%2Fsowood-stherblain.fr%2F%23contacteznous)
- [saint-herblain-westgarden](https://habx.github.io/client-orbital?manifest=data/saint-herblain-westgarden.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-saint-herblain%2Fwest-garden%2Cp84371%3F%23%25ID%25)
- [sartrouville-11-arpent](https://habx.github.io/client-orbital?manifest=data/sartrouville-11-arpent.json&interactive=true&redirection=https%3A%2F%2Fwww.quartus-residentiel.fr%2Fappartement%2Fsartrouville-08021-%25SLUG%25)
- [sartrouville-50-roosevelt](https://habx.github.io/client-orbital?manifest=data/sartrouville-50-roosevelt.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-sartrouville%2Fsequana%2Cp95301%3F%23%25ID%25)
- [sartrouville-sequana](https://habx.github.io/client-orbital?manifest=data/sartrouville-sequana.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-sartrouville%2Fsequana%2Cp95301%23%25ID%25)
- [sceaux-le-s](https://habx.github.io/client-orbital?manifest=data/sceaux-le-s.json&interactive=true)
- [sevran-montceleux](https://habx.github.io/client-orbital?manifest=data/sevran-montceleux.json&interactive=true)
- [seynod-jardin-des-sens](https://habx.github.io/client-orbital?manifest=data/seynod-jardin-des-sens.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-annecy%2Fjardin-des-sens%2Cp37301%23%25ID%25)
- [sixfourslesplages](https://habx.github.io/client-orbital?manifest=data/sixfourslesplages.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-six-fours-les-plages%2Fla-reserve%2Cp80481%23%25ID%25)
- [suresnes-atelier-lumiere](https://habx.github.io/client-orbital?manifest=data/suresnes-atelier-lumiere.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-suresnes%2Fatelier-lumiere-%2Cp83981%23%25ID%25)
- [tassin-la-demi-lune-sphere](https://habx.github.io/client-orbital?manifest=data/tassin-la-demi-lune-sphere.json&interactive=true)
- [terville-l-atelier](https://habx.github.io/client-orbital?manifest=data/terville-l-atelier.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-terville%2Fl-atelier%2Cp93171%23%25ID%25)
- [toulouse-bella-rosa](https://habx.github.io/client-orbital?manifest=data/toulouse-bella-rosa.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-toulouse%2Fbella-rosa%2Cp55301%23%25ID%25)
- [toulouse-chemin-du-loup](https://habx.github.io/client-orbital?manifest=data/toulouse-chemin-du-loup.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-toulouse%2Fokami%2Cp69701%23%25ID%25)
- [toulouse-cymea](https://habx.github.io/client-orbital?manifest=data/toulouse-cymea.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-toulouse%2Fcymea%2Cp66271%23%25ID%25)
- [tourcoing-le-residentiel](https://habx.github.io/client-orbital?manifest=data/tourcoing-le-residentiel.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-tourcoing%2Fle-residentiel%2Cp78631%23%25ID%25)
- [tours-le-calice](https://habx.github.io/client-orbital?manifest=data/tours-le-calice.json&interactive=true)
- [trelaze-simone-boisecq](https://habx.github.io/client-orbital?manifest=data/trelaze-simone-boisecq.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-trelaze%2Fateliers-lucina%2Cp34942%23%25ID%25)
- [trois-rivieres-3-rivieres](https://habx.github.io/client-orbital?manifest=data/trois-rivieres-3-rivieres.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-trois-rivieres%2Fozalee%2Cp54021%23%25ID%25)
- [vanves-issy](https://habx.github.io/client-orbital?manifest=data/vanves-issy.json&interactive=true)
- [villeneuve-ascq-fusilles](https://habx.github.io/client-orbital?manifest=data/villeneuve-ascq-fusilles.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-villeneuve-d-ascq%2Fle-cedre-d-ascq%2Cp43571%23%25ID%25)
- [villeneuve-la-garenne-asnieres](https://habx.github.io/client-orbital?manifest=data/villeneuve-la-garenne-asnieres.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-villeneuve-la-garenne%2Fvilla-en-seine%2Cp30341%23%25ID%25)
- [villepinte-greenpark](https://habx.github.io/client-orbital?manifest=data/villepinte-greenpark.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-villepinte%2Fgreen-park%2Cp89001%23%25ID%25)
- [villeurbanne-green-republique](https://habx.github.io/client-orbital?manifest=data/villeurbanne-green-republique.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-villeurbanne%2Fgreen-republique%2Cp57561%23%25ID%25)
- [villeurbanne-renaissance](https://habx.github.io/client-orbital?manifest=data/villeurbanne-renaissance.json&interactive=true&redirection=https%3A%2F%2Fwww.icade-immobilier.com%2Fprogrammes-immobiliers-neufs-villeurbanne%2Frenaissance%2Cp84051%23%25ID%25)
- [vitrolles-vitrolles-parenthese](https://habx.github.io/client-orbital?manifest=data/vitrolles-vitrolles-parenthese.json&interactive=true&redirection=https%3A%2F%2Fwww.quartus-residentiel.fr%2Fappartement%2Fvitrolles-19851-%25SLUG%25)

## Manifest format updates (from version `2.0.0` to `3.0.0`)

- Reorder the fields given the jq transformation `'{ meta, lots, views }'`
- Added field `meta.path` that defines the location of the folder containing all images assets
- Added fields `lots[].exteriorSurfaceArea`, `lots[].images`, `lots[].levels`, `lots[].name`, `lots[].slug`, `lots[].surfaceArea`, `lots[].typology`, `lots[].vat`
