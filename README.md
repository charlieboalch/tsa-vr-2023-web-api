# TSA VR 2023 Backend

### API Documentation
- Endpoint- https://phqsh.tech/vr/
- Method- GET
- Response- JSON
- Params:
  - species: string (Dog/Cat)
  - size: string (small/medium/large/xlarge)
  - gender: string (male/female/unknown)
  - age: string (baby/young/adult/senior)
  - good_with_kids: boolean (true/false)
  - good_with_animals: boolean (true/false) (includes both dogs and cats)
  - house_trained: boolean (true/false)
  - location: int (zip code)

### Example
https://phqsh.tech/vr/api?species=Dog&size=Medium&gender=Male&age=Senior&good_with_kids=true&good_with_animals=true&house_trained=true&location=40202

```json
{
  "animals": [
    {
      "id": 64559785,
      "organization_id": "IN688",
      "url": "https://www.petfinder.com/dog/buster-brown-64559785/in/fountaintown/canine-castaways-rescue-in688/?referrer_id=a55dd8a0-9e9f-4d86-8391-055839c1bee1",
      "type": "Dog",
      "species": "Dog",
      "breeds": {
        "primary": "Boxer",
        "secondary": null,
        "mixed": false,
        "unknown": false
      },
      "colors": {
        "primary": "Brown / Chocolate",
        "secondary": "White / Cream",
        "tertiary": "Black"
      },
      "age": "Senior",
      "gender": "Male",
      "size": "Medium",
      "coat": "Short",
      "attributes": {
        "spayed_neutered": true,
        "house_trained": true,
        "declawed": null,
        "special_needs": true,
        "shots_current": true
      },
      "environment": {
        "children": true,
        "dogs": true,
        "cats": true
      },
      "tags": [
        "Friendly",
        "Affectionate",
        "Loyal",
        "Gentle",
        "Smart",
        "Loves kisses",
        "Funny"
      ],
      "name": "Buster Brown",
      "description": "Meet Buster Brown!\n\nKids: Yes\nDogs: Yes\nCats: Yes\n\nHe is very loving, and ready to find his forever family!...",
      "organization_animal_id": null,
      "photos": [
        {
          "small": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776&width=100",
          "medium": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776&width=300",
          "large": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776&width=600",
          "full": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776"
        },
        {
          "small": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/1/?bust=1684456135&width=100",
          "medium": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/1/?bust=1684456135&width=300",
          "large": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/1/?bust=1684456135&width=600",
          "full": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/1/?bust=1684456135"
        },
        {
          "small": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/2/?bust=1684456135&width=100",
          "medium": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/2/?bust=1684456135&width=300",
          "large": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/2/?bust=1684456135&width=600",
          "full": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/2/?bust=1684456135"
        },
        {
          "small": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/4/?bust=1684456136&width=100",
          "medium": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/4/?bust=1684456136&width=300",
          "large": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/4/?bust=1684456136&width=600",
          "full": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/4/?bust=1684456136"
        }
      ],
      "primary_photo_cropped": {
        "small": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776&width=300",
        "medium": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776&width=450",
        "large": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776&width=600",
        "full": "https://dl5zpyw5k3jeb.cloudfront.net/photos/pets/64559785/3/?bust=1684535776"
      },
      "videos": [],
      "status": "adoptable",
      "status_changed_at": "2023-05-19T00:29:01+0000",
      "published_at": "2023-05-19T00:28:58+0000",
      "distance": 99.8044,
      "contact": {
        "email": "info@caninecastawaysrescue.org",
        "phone": null,
        "address": {
          "address1": null,
          "address2": null,
          "city": "Fountaintown",
          "state": "IN",
          "postcode": "46130",
          "country": "US"
        }
      },
      "_links": {
        "self": {
          "href": "/v2/animals/64559785"
        },
        "type": {
          "href": "/v2/types/dog"
        },
        "organization": {
          "href": "/v2/organizations/in688"
        }
      }
    }
  ],
  "pagination": {
    "count_per_page": 100,
    "total_count": 1,
    "current_page": 1,
    "total_pages": 1
  }
}
```
