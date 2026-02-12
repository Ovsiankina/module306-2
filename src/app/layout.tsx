import type { Metadata } from "next";
import { GeistSans } from "geist/font/sans";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/next";
import { Toaster } from "sonner";
import { Session, getServerSession } from "next-auth";
import { authOptions } from "@/libs/auth";

import { Navbar } from "@/components/layout/navbar";
import { Footer } from "@/components/layout/footer";
import { Providers } from "@/providers";
import "@/styles/globals.css";
import "@/styles/colors.css";
import "@/styles/animations.css";

export const metadata: Metadata = {
  title: "FoxTown Factory Stores - Mendrisio",
  description: "FoxTown Factory Stores est le paradis du shopping avec 160 boutiques et 250 marques prestigieuses. Réductions de 30% à 70% toute l'année. Jouez à notre roue de la fortune pour gagner des bons d'achat!",
};

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session: Session | null = await getServerSession(authOptions);

  return (
    <html lang="fr">
      <Providers>
        <body className={GeistSans.className}>
          <Navbar session={session} />
          <main className="pointer-events-auto">
            {children}
            <Toaster position="bottom-right" />
            <Analytics />
            <SpeedInsights />
          </main>
          <Footer />
        </Providers>
      </body>
    </html>
  );
}
