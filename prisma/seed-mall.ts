import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

async function main() {
  console.log("Seeding FoxTown Factory Stores data...\n");

  // Clear existing mall data
  await prisma.voucher.deleteMany();
  await prisma.shop.deleteMany();
  await prisma.parking.deleteMany();
  await prisma.mallInfo.deleteMany();
  await prisma.dailyPrizePool.deleteMany();

  // Create Mall Info - FoxTown
  await prisma.mallInfo.create({
    data: {
      name: "FoxTown Factory Stores",
      address: "Via Angelo Maspoli 18, 6850 Mendrisio, Switzerland",
      phone: "+41 848 828 888",
      email: "info@foxtown.com",
      openingHours: "Ouvert 7j/7 de 11h00 à 19h00",
      description: "FoxTown Factory Stores est le paradis du shopping avec 160 boutiques et 250 marques prestigieuses. Réductions de 30% à 70% toute l'année.",
      mapImage: "mall/foxtown-plan.pdf",
    },
  });
  console.log("Created: Mall Info (FoxTown)");

  // Create Shops - Real FoxTown stores from the PDF
  const shops = [
    // HIGH FASHION - Level 3 (Green)
    { name: "Prada", description: "Mode de luxe italienne, prêt-à-porter et accessoires.", category: "High Fashion", floor: 3, location: "312", websiteUrl: "https://www.prada.com", phone: "+41 91 986 64 40" },
    { name: "Gucci", description: "Maison de mode italienne de luxe.", category: "High Fashion", floor: 3, location: "339", websiteUrl: "https://www.gucci.com", phone: "+41 91 630 29 90" },
    { name: "Versace", description: "Mode italienne haut de gamme.", category: "High Fashion", floor: 3, location: "301", websiteUrl: "https://www.versace.com", phone: "+41 91 646 74 74" },
    { name: "Burberry", description: "Mode britannique de luxe.", category: "High Fashion", floor: 3, location: "343", websiteUrl: "https://www.burberry.com", phone: "+41 44 588 99 62" },
    { name: "Dolce & Gabbana", description: "Haute couture italienne.", category: "High Fashion", floor: 3, location: "339", websiteUrl: "https://www.dolcegabbana.com", phone: "+41 91 630 29 90" },
    { name: "Armani", description: "Mode italienne élégante.", category: "High Fashion", floor: 3, location: "328", websiteUrl: "https://www.armani.com", phone: "+41 91 630 06 17" },
    { name: "Boss", description: "Mode allemande haut de gamme.", category: "High Fashion", floor: 3, location: "326", websiteUrl: "https://www.hugoboss.com", phone: "+41 91 630 26 40" },
    { name: "Michael Kors", description: "Accessoires et prêt-à-porter américain.", category: "High Fashion", floor: 2, location: "214", websiteUrl: "https://www.michaelkors.com", phone: "+41 91 646 81 00" },
    { name: "Coach", description: "Maroquinerie et accessoires de luxe américains.", category: "High Fashion", floor: 2, location: "201", websiteUrl: "https://www.coach.com", phone: "+41 91 646 01 86" },
    { name: "Furla", description: "Maroquinerie italienne.", category: "High Fashion", floor: 3, location: "316", websiteUrl: "https://www.furla.com", phone: "+41 91 646 36 01" },

    // SPORTSWEAR - Level 2 (Blue)
    { name: "Nike Factory Store", description: "Chaussures, vêtements et équipements de sport.", category: "Sportswear", floor: 1, location: "111", websiteUrl: "https://www.nike.com", phone: "+41 91 640 40 60" },
    { name: "Puma", description: "Articles de sport et lifestyle.", category: "Sportswear", floor: 2, location: "245", websiteUrl: "https://www.puma.com", phone: "+41 91 630 27 01" },
    { name: "The North Face", description: "Vêtements et équipements outdoor.", category: "Sportswear", floor: 2, location: "235", websiteUrl: "https://www.thenorthface.com", phone: "+41 91 646 23 09" },
    { name: "Salomon", description: "Équipements de sports de montagne.", category: "Sportswear", floor: 2, location: "255", websiteUrl: "https://www.salomon.com", phone: "+41 91 646 77 44" },
    { name: "New Balance", description: "Chaussures de sport et lifestyle.", category: "Sportswear", floor: 2, location: "256", websiteUrl: "https://www.newbalance.com", phone: "+41 91 646 11 01" },
    { name: "Mammut", description: "Équipements alpins suisses.", category: "Sportswear", floor: 2, location: "254", websiteUrl: "https://www.mammut.com", phone: "+41 91 630 12 44" },
    { name: "Oakley", description: "Lunettes et équipements sportifs.", category: "Sportswear", floor: 2, location: "213", websiteUrl: "https://www.oakley.com", phone: "+41 91 646 49 50" },

    // CASUALWEAR - Level 2 (Blue)
    { name: "Tommy Hilfiger", description: "Mode américaine décontractée.", category: "Casualwear", floor: 2, location: "242", websiteUrl: "https://www.tommy.com", phone: "+41 91 630 20 54" },
    { name: "Calvin Klein", description: "Mode américaine contemporaine.", category: "Casualwear", floor: 2, location: "247", websiteUrl: "https://www.calvinklein.com", phone: "+41 91 646 02 50" },
    { name: "Lacoste", description: "Mode sportswear française.", category: "Casualwear", floor: 1, location: "129", websiteUrl: "https://www.lacoste.com", phone: "+41 91 646 11 24" },
    { name: "Levi's & Dockers", description: "Jeans et vêtements décontractés.", category: "Casualwear", floor: 1, location: "146", websiteUrl: "https://www.levi.com", phone: "+41 91 630 29 85" },
    { name: "Guess", description: "Mode américaine tendance.", category: "Casualwear", floor: 2, location: "241", websiteUrl: "https://www.guess.com", phone: "+41 91 630 26 50" },
    { name: "Diesel", description: "Mode italienne denim et casual.", category: "Casualwear", floor: 1, location: "113", websiteUrl: "https://www.diesel.com", phone: "+41 91 646 02 46" },
    { name: "Replay", description: "Denim italien premium.", category: "Casualwear", floor: 2, location: "221", websiteUrl: "https://www.replay.it", phone: "+41 91 630 27 14" },
    { name: "Napapijri", description: "Mode outdoor et streetwear.", category: "Casualwear", floor: 3, location: "353", websiteUrl: "https://www.napapijri.com", phone: "+41 91 646 74 83" },
    { name: "Timberland", description: "Chaussures et vêtements outdoor.", category: "Casualwear", floor: 2, location: "261", websiteUrl: "https://www.timberland.com", phone: "+41 91 630 23 55" },

    // FOOTWEAR - Level 2 (Blue)
    { name: "Geox", description: "Chaussures respirantes italiennes.", category: "Footwear", floor: 1, location: "155", websiteUrl: "https://www.geox.com", phone: "+41 91 630 22 24" },
    { name: "Ecco", description: "Chaussures confort danoises.", category: "Footwear", floor: 2, location: "238", websiteUrl: "https://www.ecco.com", phone: "+41 91 646 08 88" },
    { name: "Clarks", description: "Chaussures britanniques traditionnelles.", category: "Footwear", floor: 1, location: "128", websiteUrl: "https://www.clarks.com", phone: "+41 91 630 08 01" },
    { name: "Skechers", description: "Chaussures confort américaines.", category: "Footwear", floor: 2, location: "228", websiteUrl: "https://www.skechers.com", phone: "+41 91 646 01 60" },
    { name: "Bally", description: "Chaussures de luxe suisses.", category: "Footwear", floor: 3, location: "305", websiteUrl: "https://www.bally.com", phone: "+41 91 646 73 45" },
    { name: "Tod's - Hogan", description: "Chaussures italiennes de luxe.", category: "Footwear", floor: 3, location: "309", websiteUrl: "https://www.tods.com", phone: "+41 91 646 92 15" },
    { name: "Jimmy Choo", description: "Chaussures de luxe.", category: "Footwear", floor: 2, location: "206", websiteUrl: "https://www.jimmychoo.com", phone: "+41 91 646 18 20" },
    { name: "Ugg", description: "Bottes et chaussures confort.", category: "Footwear", floor: 2, location: "210", websiteUrl: "https://www.ugg.com", phone: "+41 91 630 00 71" },
    { name: "Vans", description: "Chaussures skateboard et lifestyle.", category: "Footwear", floor: 2, location: "252", websiteUrl: "https://www.vans.com", phone: "+41 91 630 02 93" },

    // WATCHES & JEWELLERY
    { name: "Swarovski", description: "Cristaux et bijoux autrichiens.", category: "Watches & Jewellery", floor: 1, location: "117", websiteUrl: "https://www.swarovski.com", phone: "+41 91 646 01 78" },
    { name: "Swatch", description: "Montres suisses.", category: "Watches & Jewellery", floor: 3, location: "345", websiteUrl: "https://www.swatch.com", phone: "+41 91 646 90 09" },
    { name: "Hour Passion", description: "Montres multimarques.", category: "Watches & Jewellery", floor: 3, location: "317", websiteUrl: "https://www.hourpassion.com", phone: "+41 91 646 14 44" },

    // ACCESSORIES
    { name: "Samsonite", description: "Bagages et accessoires de voyage.", category: "Accessories", floor: 2, location: "204", websiteUrl: "https://www.samsonite.com", phone: "+41 91 630 21 70" },
    { name: "Coccinelle", description: "Maroquinerie italienne.", category: "Accessories", floor: 3, location: "310", websiteUrl: "https://www.coccinelle.com", phone: "+41 91 630 00 24" },
    { name: "Longchamp", description: "Maroquinerie française de luxe.", category: "High Fashion", floor: 3, location: "322", websiteUrl: "https://www.longchamp.com", phone: "+41 91 600 35 30" },

    // HOME
    { name: "Le Creuset", description: "Ustensiles de cuisine français.", category: "Home", floor: 0, location: "010", websiteUrl: "https://www.lecreuset.com", phone: "+41 91 646 83 37" },
    { name: "Villeroy & Boch", description: "Arts de la table et céramique.", category: "Home", floor: 2, location: "220", websiteUrl: "https://www.villeroy-boch.com", phone: "+41 91 630 26 20" },
    { name: "WMF", description: "Ustensiles de cuisine allemands.", category: "Home", floor: 2, location: "258", websiteUrl: "https://www.wmf.com", phone: "+41 91 646 21 25" },

    // BEAUTY
    { name: "Kiko Milano", description: "Cosmétiques italiens.", category: "Beauty", floor: 2, location: "274", websiteUrl: "https://www.kikocosmetics.com", phone: "+41 91 630 02 55" },

    // FOOD & DRINKS - Level 1 (Red) and others
    { name: "Lindt", description: "Chocolat suisse premium.", category: "Food & Drinks", floor: 1, location: "132", websiteUrl: "https://www.lindt.com", phone: "+41 91 914 48 58" },
    { name: "Starbucks", description: "Café et boissons.", category: "Food & Drinks", floor: 2, location: "278", websiteUrl: "https://www.starbucks.com", phone: "+41 91 646 58 06" },
    { name: "Wood Avenue", description: "Restaurant italien.", category: "Food & Drinks", floor: 1, location: "170", websiteUrl: "", phone: "+41 91 646 03 56" },
    { name: "Pizzeria", description: "Pizza italienne.", category: "Food & Drinks", floor: 1, location: "137", websiteUrl: "", phone: "+41 91 630 27 81" },
    { name: "Maui Poke", description: "Cuisine hawaïenne.", category: "Food & Drinks", floor: 1, location: "169", websiteUrl: "", phone: "+41 91 222 99 01" },
    { name: "Chalet Suisse", description: "Spécialités suisses.", category: "Food & Drinks", floor: 2, location: "259", websiteUrl: "", phone: "+41 91 630 28 89" },

    // CHILDRENSWEAR
    { name: "Kid Space", description: "Mode enfants multimarques.", category: "Childrenswear", floor: 2, location: "222", websiteUrl: "", phone: "+41 91 646 80 62" },
    { name: "Jacadi Paris", description: "Mode enfants française.", category: "Childrenswear", floor: 3, location: "358", websiteUrl: "https://www.jacadi.com", phone: "+41 91 646 38 88" },

    // SERVICES
    { name: "Casino Admiral", description: "Casino avec machines à sous, roulette et poker.", category: "Services", floor: 1, location: "1011", websiteUrl: "", phone: "+41 91 640 50 20" },
    { name: "The Sense Gallery", description: "Première réalité multisensorielle en Suisse.", category: "Services", floor: 1, location: "166", websiteUrl: "", phone: "+41 91 610 99 63" },
    { name: "Tichange", description: "Bureau de change.", category: "Services", floor: 1, location: "120", websiteUrl: "", phone: "+41 91 630 00 61" },
  ];

  for (const shop of shops) {
    const createdShop = await prisma.shop.create({
      data: {
        ...shop,
        logo: "",
        image: "",
        openingHours: "11h00 - 19h00",
        isActive: true,
      },
    });

    // Create sample vouchers for some shops (fashion and sportswear)
    if (["High Fashion", "Sportswear", "Casualwear"].includes(shop.category)) {
      await prisma.voucher.create({
        data: {
          shopId: createdShop.id,
          code: `FOX-${shop.name.toUpperCase().replace(/[^A-Z]/g, "").slice(0, 6)}-${Math.random().toString(36).substring(2, 8).toUpperCase()}`,
          value: [10, 20, 30, 40][Math.floor(Math.random() * 4)],
          description: `Bon d'achat ${shop.name}`,
          expiresAt: new Date(Date.now() + 90 * 24 * 60 * 60 * 1000),
        },
      });
    }

    console.log(`Created: ${shop.name} (Level ${shop.floor})`);
  }

  // Create Parking areas - FoxPark
  const parkings = [
    {
      name: "FoxPark - Parking Principal",
      totalSpaces: 2500,
      availableSpaces: 1250,
      floor: "Extérieur",
      isOpen: true,
    },
    {
      name: "Parking Souterrain Nord",
      totalSpaces: 500,
      availableSpaces: 234,
      floor: "-1",
      isOpen: true,
    },
    {
      name: "Parking Souterrain Sud",
      totalSpaces: 400,
      availableSpaces: 156,
      floor: "-1",
      isOpen: true,
    },
  ];

  for (const parking of parkings) {
    await prisma.parking.create({ data: parking });
    console.log(`Created: ${parking.name}`);
  }

  // Initialize today's prize pool
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  await prisma.dailyPrizePool.create({
    data: {
      date: today,
      totalPrizes: 10,
      prizesWon: 0,
    },
  });
  console.log("Created: Today's prize pool");

  console.log("\n✅ FoxTown seeding completed!");
  console.log(`   - ${shops.length} shops created`);
  console.log(`   - ${parkings.length} parking areas created`);
  console.log(`   - Vouchers created for fashion/sportswear shops`);
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
