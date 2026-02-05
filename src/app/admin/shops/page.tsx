import { getServerSession } from "next-auth/next";
import { authOptions } from "@/libs/auth";
import { redirect } from "next/navigation";
import { prisma } from "@/libs/db";
import Link from "next/link";
import { Plus, Edit, Trash2, ExternalLink, ArrowLeft } from "lucide-react";
import { revalidatePath } from "next/cache";

export const metadata = {
  title: "Gestion des Boutiques - Admin FoxTown",
};

async function deleteShop(formData: FormData) {
  "use server";
  const id = formData.get("id") as string;
  await prisma.shop.update({
    where: { id },
    data: { isActive: false },
  });
  revalidatePath("/admin/shops");
}

async function activateShop(formData: FormData) {
  "use server";
  const id = formData.get("id") as string;
  await prisma.shop.update({
    where: { id },
    data: { isActive: true },
  });
  revalidatePath("/admin/shops");
}

export default async function AdminShopsPage() {
  const session = await getServerSession(authOptions);

  if (!session?.user?.isAdmin) {
    redirect("/login?error=unauthorized");
  }

  const shops = await prisma.shop.findMany({
    orderBy: [{ isActive: "desc" }, { name: "asc" }],
  });

  const categories = [...new Set(shops.map((s) => s.category))].sort();

  return (
    <section className="container mx-auto px-4 py-12">
      <div className="flex items-center gap-4 mb-8">
        <Link
          href="/admin"
          className="p-2 hover:bg-gray-100 rounded-lg transition"
        >
          <ArrowLeft className="w-5 h-5" />
        </Link>
        <div className="flex-1">
          <h1 className="text-3xl font-bold text-gray-900">Gestion des Boutiques</h1>
          <p className="text-gray-600">{shops.filter((s) => s.isActive).length} boutiques actives</p>
        </div>
        <Link
          href="/admin/shops/new"
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
        >
          <Plus className="w-5 h-5" />
          Ajouter
        </Link>
      </div>

      {/* Filters */}
      <div className="bg-white rounded-xl shadow-lg p-4 mb-6">
        <div className="flex flex-wrap gap-2">
          <span className="text-sm font-medium text-gray-500 self-center mr-2">Catégories:</span>
          {categories.map((category) => (
            <span
              key={category}
              className="px-3 py-1 bg-gray-100 rounded-full text-sm"
            >
              {category} ({shops.filter((s) => s.category === category && s.isActive).length})
            </span>
          ))}
        </div>
      </div>

      {/* Shops Table */}
      <div className="bg-white rounded-xl shadow-lg overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Boutique
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Catégorie
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Niveau
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Emplacement
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Statut
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {shops.map((shop) => (
              <tr key={shop.id} className={!shop.isActive ? "bg-gray-50 opacity-60" : ""}>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex items-center gap-3">
                    <div>
                      <p className="font-medium text-gray-900">{shop.name}</p>
                      {shop.websiteUrl && (
                        <a
                          href={shop.websiteUrl}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="text-xs text-blue-600 hover:underline flex items-center gap-1"
                        >
                          Site web <ExternalLink className="w-3 h-3" />
                        </a>
                      )}
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className="px-2 py-1 bg-gray-100 rounded text-sm">
                    {shop.category}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  Level {shop.floor}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {shop.location}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  {shop.isActive ? (
                    <span className="px-2 py-1 bg-green-100 text-green-800 rounded-full text-xs font-medium">
                      Actif
                    </span>
                  ) : (
                    <span className="px-2 py-1 bg-red-100 text-red-800 rounded-full text-xs font-medium">
                      Inactif
                    </span>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-right">
                  <div className="flex items-center justify-end gap-2">
                    <Link
                      href={`/admin/shops/${shop.id}`}
                      className="p-2 text-gray-500 hover:text-blue-600 hover:bg-blue-50 rounded transition"
                    >
                      <Edit className="w-4 h-4" />
                    </Link>
                    {shop.isActive ? (
                      <form action={deleteShop}>
                        <input type="hidden" name="id" value={shop.id} />
                        <button
                          type="submit"
                          className="p-2 text-gray-500 hover:text-red-600 hover:bg-red-50 rounded transition"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </form>
                    ) : (
                      <form action={activateShop}>
                        <input type="hidden" name="id" value={shop.id} />
                        <button
                          type="submit"
                          className="px-3 py-1 text-xs bg-green-100 text-green-700 rounded hover:bg-green-200 transition"
                        >
                          Réactiver
                        </button>
                      </form>
                    )}
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
