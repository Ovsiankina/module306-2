"use client";

import { forwardRef } from "react";
import Link from "next/link";

import { cn } from "@/lib/utils";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";

export function LinksDesktop() {
  return (
    <NavigationMenu>
      <NavigationMenuList>
        <NavigationMenuItem>
          <NavigationMenuTrigger>Boutiques</NavigationMenuTrigger>
          <NavigationMenuContent>
            <ul className="grid gap-3 p-4 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]">
              <li className="row-span-3">
                <NavigationMenuLink asChild>
                  <Link
                    className="flex flex-col justify-end w-full h-full p-6 no-underline rounded-md outline-none select-none bg-gradient-to-br from-orange-500 to-red-600 focus:shadow-md"
                    href="/#boutiques"
                  >
                    <div className="mt-4 mb-1 text-sm font-medium text-white">
                      160 BOUTIQUES
                    </div>
                    <p className="text-sm leading-tight text-orange-100">
                      250 marques prestigieuses sur 4 niveaux. Réductions 30-70%.
                    </p>
                  </Link>
                </NavigationMenuLink>
              </li>
              <ListItem href="/#boutiques" title="High Fashion">
                Prada, Gucci, Versace, Burberry, Armani et plus encore.
              </ListItem>
              <ListItem href="/#boutiques" title="Sportswear">
                Nike, Puma, The North Face, Salomon, New Balance.
              </ListItem>
              <ListItem href="/#boutiques" title="Food & Drinks">
                9 bars et restaurants - Lindt, Starbucks, restaurants italiens.
              </ListItem>
            </ul>
          </NavigationMenuContent>
        </NavigationMenuItem>

        <NavigationMenuItem>
          <Link href="/plan" legacyBehavior passHref>
            <NavigationMenuLink className={navigationMenuTriggerStyle()}>
              Plan
            </NavigationMenuLink>
          </Link>
        </NavigationMenuItem>

        <NavigationMenuItem>
          <Link href="/parking" legacyBehavior passHref>
            <NavigationMenuLink className={navigationMenuTriggerStyle()}>
              Parking
            </NavigationMenuLink>
          </Link>
        </NavigationMenuItem>
      </NavigationMenuList>
    </NavigationMenu>
  );
}

const ListItem = forwardRef<
  React.ElementRef<"a">,
  Omit<React.ComponentPropsWithoutRef<"a">, "href"> & { href: string }
>(({ className, title, children, ...props }, ref) => {
  return (
    <li>
      <NavigationMenuLink asChild>
        <Link
          ref={ref}
          className={cn(
            "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground hover:bg-background-tertiary",
            className
          )}
          {...props}
        >
          <div className="text-sm font-medium leading-none text-[]">
            {title}
          </div>
          <p className="text-sm leading-snug line-clamp-2 text-muted-foreground text-color-secondary">
            {children}
          </p>
        </Link>
      </NavigationMenuLink>
    </li>
  );
});
ListItem.displayName = "ListItem";
