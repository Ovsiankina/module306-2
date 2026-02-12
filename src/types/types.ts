export interface EnrichedOrders {
  name: string;
  email: string;
  phone: string | null;
  address: AddressDocument;
  products: any[];
  orderId: string;
  total_price: number;
  orderNumber: string;
  expectedDeliveryDate: Date;
  purchaseDate: string;
  _id: string;
}

export interface EnrichedProducts {
  name: string;
  category: string;
  image: string[];
  price: number;
  purchased: boolean;
  color: string;
  size: string;
  quantity: number;
  productId: string;
  _id: string;
  variantId: string;
}

export interface OrdersDocument {
  id: string;
  userId: string;
  orders: OrderDocument[];
}

export interface OrderDocument {
  name: string;
  email: string;
  phone: number;
  address: AddressDocument;
  products: ProductsDocument[];
  orderId: string;
  purchaseDate: Date;
  expectedDeliveryDate: Date;
  total_price: number;
  orderNumber: string;
  _id: string;
}

export interface AddressDocument {
  city: string;
  country: string;
  line1: string;
  line2: string;
  postal_code: string;
  state: string;
}

export interface ProductsDocument {
  productId: string;
  image: string;
  color: string;
  size: string;
  quantity: number;
  _id: string;
}

export interface FavoritesDocument {
  id: string;
  userId: string;
  favorites: string[];
}

export interface ItemDocument {
  productId: string;
  color: string;
  size: string;
  quantity: number;
  variantId: string;
  price: number;
}

export interface ProductDocument {
  id: string;
  _id: string; // Alias for backwards compatibility
  name: string;
  description: string;
  price: number;
  category: string;
  sizes: string[];
  image: string[];
  variants: VariantsDocument[];
  quantity?: number;
  productId?: string;
  purchased?: boolean;
}

export interface VariantsDocument {
  priceId: string;
  color: string;
  images: string[];
}

export interface UserDocument {
  id: string;
  _id: string; // Alias for backwards compatibility
  email: string;
  password: string;
  name: string;
  phone: string;
  address?: AddressDocument;
  image?: string;
  isAdmin?: boolean;
  createdAt: Date;
  updatedAt: Date;
}
