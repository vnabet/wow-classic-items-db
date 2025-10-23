# Documentation Auctionator – Structure de la base de données LUA

## Structure générale

```lua
AUCTIONATOR_PRICE_DATABASE = {
["__dbversion"] = 8,
["Thunderstrike Alliance"] = {
["gr:10023:of Power"] = {
["a"] = { ... },
["l"] = { ... },
["h"] = { ... },
["m"] = ...,
},
...
},
...
}
```

---

## Description des champs

### Au niveau racine

- **AUCTIONATOR_PRICE_DATABASE**  
  Table qui contient toutes les données de prix des scans Auctionator sur tous les serveurs/factions.
- **"\_\_dbversion"**  
  Version du format de base de données Auctionator.
- **"[NomDuServeur Faction]"**  
  Clé regroupant tous les objets d'une faction sur un serveur particulier (ex: `"Thunderstrike Alliance"`).

---

### Au niveau objet

#### Clé objet : `"gr:xxxx:of Suffix"` ou `"xxxx"`

Indique l'objet concerné (avec ou sans suffixe).

#### Champs :

| Champ   | Fonction                                                                                                                     | Exemple                        |
| ------- | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| **"a"** | Quantité maximale de l’objet vue lors de chaque scan, indexée par le nombre de jours (_daysSinceZero_) depuis le 01/01/2020. | `["a"] = { ["2114"] = 2 }`     |
| **"l"** | *LowestLow* : Prix le plus bas observé sur la journée (stocké seulement si différent de "h").                                | `["l"] = { ["2114"] = 20853 }` |
| **"h"** | *HighestLow* : Plus haut des prix les plus bas vus dans les scans du jour.                                                   | `["h"] = { ["2114"] = 20854 }` |
| **"m"** | Dernier prix minimal observé utilisé comme référence pour l’addon.                                                           | `["m"] = 20853`                |

---

### Sur l’index des sous-champs

- Tous les index ("2114", "2100", etc.) correspondent au nombre de jours depuis le **1er janvier 2020 à minuit** (`SCAN_DAY_0` dans le code source).
- Pour convertir un index en date réelle :

`Date réelle = 1er janvier 2020 + index × 1 jour`

Par exemple, `index 2114` = 17 octobre 2025.

---

## Synthèse "l" vs "h"

- Plusieurs scans le même jour :
- **"l"** = plus bas prix vu de la journée.
- **"h"** = plus haut prix des bas vus sur la journée.
- Si elles sont différentes : stocke les deux, sinon stocke juste "h".

---

## Exemple concret

```lua
["gr:15548:of Power"] = {
["a"] = { ["2114"] = 2 },
["l"] = { ["2114"] = 20853 },
["h"] = { ["2114"] = 20854 },
["m"] = 20853,
}
["10023"] = {
["a"] = { ["2114"] = 11, ["2102"] = 5, ["2111"] = 7, ["2100"] = 1 },
["l"] = { ["2114"] = 69979, ["2100"] = 99989 },
["h"] = { ["2114"] = 69984, ["2102"] = 89990, ["2111"] = 77462, ["2100"] = 99990 },
["m"] = 69979,
}
```

---

## Conversion index → date (exemple Lua)

```lua
local scan_day_0 = os.time{year=2020, month=1, day=1, hour=0}
local index = 2114
local date = os.date("%d/%m/%Y", scan_day_0 + index*86400)
```

---

## Formats des clés d'objet dans la base Auctionator

| Format                   | Usage                                 | Exemple               | Type d’objet                                        |
| ------------------------ | ------------------------------------- | --------------------- | --------------------------------------------------- |
| `<itemID>`               | Objet simple                          | `13023`               | Item classique, composant, recette                  |
| `gr:<itemID>:<suffixe>`  | Générique à suffixe (random property) | `gr:10225:of the Owl` | Équipement ou loot à propriétés variables (suffixe) |
| `g:<itemID>:<itemLevel>` | Gear haut niveau (Retail/Moderne)     | `g:19019:233`         | Équipement avec niveau d'objet élevé (Retail only)  |
| `p:<battlePetID>`        | Mascotte de combat (Pet)              | `p:728`               | Mascotte WoW                                        |

---

## Liste des suffixes

```lua
Auctionator.Utilities.SuffixStringIDTOSuffixString = {
[1] = "of Agility",
[2] = "of Arcane Resistance",
[3] = "of Arcane Wrath",
[4] = "of Beast Slaying",
[5] = "of Blocking",
[6] = "of Concentration",
[7] = "of Critical Strike",
[8] = "of Defense",
[9] = "of Eluding",
[10] = "of Fiery Wrath",
[11] = "of Fire Resistance",
[12] = "of Frost Resistance",
[13] = "of Frozen Wrath",
[14] = "of Healing",
[15] = "of Holy Wrath",
[16] = "of Intellect",
[17] = "of Marksmanship",
[18] = "of Nature Resistance",
[19] = "of Nature's Wrath",
[20] = "of Power",
[21] = "of Proficiency",
[22] = "of Quality",
[23] = "of Regeneration",
[24] = "of Restoration",
[25] = "of Retaliation",
[26] = "of Shadow Resistance",
[27] = "of Shadow Wrath",
[28] = "of Sorcery",
[29] = "of Spirit",
[30] = "of Stamina",
[31] = "of Strength",
[32] = "of Striking",
[33] = "of Toughness",
[34] = "of Twain",
[35] = "of the Bear",
[36] = "of the Boar",
[37] = "of the Eagle",
[38] = "of the Falcon",
[39] = "of the Gorilla",
[40] = "of the Monkey",
[41] = "of the Owl",
[42] = "of the Tiger",
[43] = "of the Whale",
[44] = "of the Wolf",
}
```
