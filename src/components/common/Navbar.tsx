"use client";

import Link from "next/link";
import { useState } from "react";
import { Session } from "next-auth";
import { LinksDesktop } from "./LinksDesktop";
import { UserMenu } from "./UserMenu";
import { Dialog, DialogTrigger } from "@/components/ui/dialog";
import dynamic from "next/dynamic";
import { Store, Map, Car, Gift } from "lucide-react";

const EditProfile = dynamic(() => import("./EditProfile"), {
  ssr: false,
});

const SignOutButton = dynamic(() => import("../account/SignOutButton"), {
  ssr: false,
});

interface NavbarProps {
  session: Session | null;
}

export const Navbar = ({ session }: NavbarProps) => {
  const [isHeaderOpen, setIsHeaderOpen] = useState(false);

  const toggleHeader = () => {
    document.body.style.overflow = "auto";
    setIsHeaderOpen(!isHeaderOpen);
  };

  const linksData = [
    { path: "/", name: "Accueil", icon: Store },
    { path: "/plan", name: "Plan du centre", icon: Map },
    { path: "/parking", name: "Parking", icon: Car },
    { path: "/#jeu", name: "Jeu", icon: Gift },
  ];

  const authLinks = () => {
    if (session?.user) {
      return (
        <>
          <li className="flex lg:hidden">
            <Dialog>
              <DialogTrigger asChild>
                <button className="flex items-center w-full h-full px-4 py-2">
                  <svg
                    data-testid="geist-icon"
                    height="16"
                    strokeLinejoin="round"
                    viewBox="0 0 16 16"
                    width="16"
                    className="mr-2"
                    style={{ color: "currentColor" }}
                  >
                    <path
                      fillRule="evenodd"
                      clipRule="evenodd"
                      d="M7.75 0C5.95507 0 4.5 1.45507 4.5 3.25V3.75C4.5 5.54493 5.95507 7 7.75 7H8.25C10.0449 7 11.5 5.54493 11.5 3.75V3.25C11.5 1.45507 10.0449 0 8.25 0H7.75ZM6 3.25C6 2.2835 6.7835 1.5 7.75 1.5H8.25C9.2165 1.5 10 2.2835 10 3.25V3.75C10 4.7165 9.2165 5.5 8.25 5.5H7.75C6.7835 5.5 6 4.7165 6 3.75V3.25ZM2.5 14.5V13.1709C3.31958 11.5377 4.99308 10.5 6.82945 10.5H9.17055C11.0069 10.5 12.6804 11.5377 13.5 13.1709V14.5H2.5ZM6.82945 9C4.35483 9 2.10604 10.4388 1.06903 12.6857L1 12.8353V13V15.25V16H1.75H14.25H15V15.25V13V12.8353L14.931 12.6857C13.894 10.4388 11.6452 9 9.17055 9H6.82945Z"
                      fill="currentColor"
                    ></path>
                  </svg>
                  <span>Mon profil</span>
                </button>
              </DialogTrigger>
              <EditProfile />
            </Dialog>
          </li>

          <li className="items-center justify-center hidden lg:flex">
            <UserMenu fastSession={session} />
          </li>
        </>
      );
    } else {
      return (
        <li className="flex items-center justify-center">
          <Link
            href="/login"
            onClick={() => setIsHeaderOpen(false)}
            className="text-sm px-4 py-2 transition-all lg:text-[#A1A1A1] hover:text-[#EDEDED] font-medium"
          >
            Connexion
          </Link>
        </li>
      );
    }
  };

  return (
    <header className="pointer-events-auto w-full px-3.5 gap-4 xs:px-6 sm:px-12 py-6 flex items-center justify-between bg-background-secondary border-b border-solid border-border-primary">
      <button
        onClick={() => {
          toggleHeader();
          document.body.style.overflow = "hidden";
        }}
        className="flex px-4 py-2 lg:hidden"
      >
        <svg
          width="24"
          height="24"
          viewBox="0 0 15 15"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M1.5 3C1.22386 3 1 3.22386 1 3.5C1 3.77614 1.22386 4 1.5 4H13.5C13.7761 4 14 3.77614 14 3.5C14 3.22386 13.7761 3 13.5 3H1.5ZM1 7.5C1 7.22386 1.22386 7 1.5 7H13.5C13.7761 7 14 7.22386 14 7.5C14 7.77614 13.7761 8 13.5 8H1.5C1.22386 8 1 7.77614 1 7.5ZM1 11.5C1 11.2239 1.22386 11 1.5 11H13.5C13.7761 11 14 11.2239 14 11.5C14 11.7761 13.7761 12 13.5 12H1.5C1.22386 12 1 11.7761 1 11.5Z"
            fill="currentColor"
            fillRule="evenodd"
            clipRule="evenodd"
          ></path>
        </svg>
      </button>

      <div
        className={`fixed top-0 left-0 h-screen w-full bg-background-secondary py-6 px-3.5 xs:px-6 transition ease duration-200 z-20 translate-x-0 ${isHeaderOpen ? "translate-x-0" : "translate-x-hide"}`}
      >
        <ul className="flex justify-between text-sm gap-9">
          <li>
            <button
              onClick={() => {
                toggleHeader();
                document.body.style.overflow = "auto";
              }}
              className="px-4 py-2"
            >
              <svg
                width="24"
                height="24"
                viewBox="0 0 15 15"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z"
                  fill="currentColor"
                  fillRule="evenodd"
                  clipRule="evenodd"
                ></path>
              </svg>
            </button>
          </li>

          {authLinks()}
        </ul>

        <div className="flex items-center justify-center h-full max-h-[90%]">
          <ul className="flex flex-col justify-between text-sm gap-9">
            {linksData.map((link, index) => {
              const Icon = link.icon;
              return (
                <li key={index} className="flex items-center justify-center">
                  <Link
                    href={link.path}
                    onClick={toggleHeader}
                    className="flex items-center gap-2"
                  >
                    <Icon className="w-5 h-5" />
                    {link.name}
                  </Link>
                </li>
              );
            })}
            {session?.user ? (
              <>
                <li className="flex items-center">
                  <Link
                    className="flex items-center w-full h-full px-4 py-2"
                    onClick={toggleHeader}
                    href="/mes-bons"
                  >
                    <Gift className="w-4 h-4 mr-2" />
                    <span>Mes bons d&apos;achat</span>
                  </Link>
                </li>
                <li className="px-4 py-2 w-[110px] mx-auto">
                  <SignOutButton />
                </li>
              </>
            ) : (
              ""
            )}
          </ul>
        </div>
      </div>

      <Link href="/" className="flex items-center gap-2">
        <Store className="w-6 h-6 text-orange-500" />
        <span className="font-bold text-lg hidden sm:inline">FoxTown</span>
      </Link>

      <ul className="justify-between hidden gap-2 text-sm lg:flex">
        {authLinks()}
        <li>
          <LinksDesktop />
        </li>
      </ul>

      <div className="flex items-center gap-4">
        {session?.user && (
          <Link
            href="/mes-bons"
            className="hidden sm:flex items-center gap-2 text-sm text-purple-400 hover:text-purple-300 transition"
          >
            <Gift className="w-5 h-5" />
            <span>Mes bons</span>
          </Link>
        )}
        {!session?.user && (
          <Link
            href="/login"
            className="hidden lg:flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition text-sm font-medium"
          >
            Se connecter
          </Link>
        )}
      </div>
    </header>
  );
};
