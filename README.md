# Dioxus Example: An e-commerce site using the FakeStoreAPI

This example app is a fullstack web application leveraging the FakeStoreAPI and [Tailwind CSS](https://tailwindcss.com/).

![Demo Image](demo.png)

# Development

1. Run the following commands to serve the application:

```bash
dx serve
```

Note that in Dioxus 0.7, the Tailwind watcher is initialized automatically if a `tailwind.css` file is find in your app's root.

# Status

This is a work in progress. The following features are currently implemented:

- [x] A homepage with a list of products dynamically fetched from the FakeStoreAPI (rendered using SSR)
- [x] A product detail page with details about a product (rendered using LiveView)
- [ ] A cart page
- [ ] A checkout page
- [ ] A login page

---

## TODO List - Shopping Center Website (Client Requirements)

Based on the client specifications in `documentation/306-DEVA.pdf`, the following features need to be implemented for the FoxTown Shopping Center website:

### 🏪 Core Features

#### 1. Shop Directory & Information
- [ ] **Replace FakeStoreAPI with real shopping center data**
  - [ ] Create database schema for shops (name, category, floor/level, store number, phone, website)
  - [ ] Import all 160 stores from the FoxTown plan (documentation/Plan centre commercial.pdf)
  - [ ] Categorize shops by type (High Fashion, Casualwear, Sportswear, Footwear, etc.)
- [ ] **Shop listing page**
  - [ ] Display all shops in the center with filtering by category
  - [ ] Show shop details (location, contact info, opening hours)
  - [ ] Add external links to each shop's official website
- [ ] **Shop detail page**
  - [ ] Individual page for each shop with full information
  - [ ] Display location on shopping center map
  - [ ] Link to official website

#### 2. Interactive Game System 🎮
- [ ] **Design and implement a game for voucher prizes**
  - [ ] Choose game type (wheel of fortune, scratch card, slot machine, etc.)
  - [ ] Game logic implementation
- [ ] **User authentication system**
  - [ ] User registration
  - [ ] User login/logout
  - [ ] Session management
- [ ] **Game rules implementation**
  - [ ] One game per day per user
  - [ ] Second chance if user loses first round
  - [ ] Maximum 10 prizes distributed per day
  - [ ] Prize distribution across different shops
- [ ] **Prize management**
  - [ ] Database for available vouchers
  - [ ] Prize redemption system
  - [ ] Admin interface to manage prizes

#### 3. Shopping Center Map 🗺️
- [ ] **Interactive map display**
  - [ ] Integrate the 4-level FoxTown map (Levels 0, 1, 2, 3)
  - [ ] Display all shop locations by floor
  - [ ] Search functionality to locate specific shops
  - [ ] Visual navigation (entrances, exits, facilities)
- [ ] **Map features**
  - [ ] Click on shop to see details
  - [ ] Filter by shop category
  - [ ] Show current location (if possible)

#### 4. Content Management System (CMS) 📝
- [ ] **Admin panel for collaborators**
  - [ ] Simple, user-friendly interface (no technical knowledge required)
  - [ ] WYSIWYG editor for content updates
- [ ] **Editable content**
  - [ ] Shop information updates
  - [ ] News and announcements
  - [ ] Event management
  - [ ] Banner/promotional content
- [ ] **User roles and permissions**
  - [ ] Admin role (full access)
  - [ ] Editor role (content updates only)
  - [ ] Authentication for admin area

#### 5. Parking Information 🅿️
- [ ] **Parking availability display**
  - [ ] Real-time or regularly updated parking space counts
  - [ ] Multiple parking areas/zones
  - [ ] Visual indicators (full, available spaces, etc.)
- [ ] **Integration considerations**
  - [ ] API or data source for parking information
  - [ ] Update mechanism (manual or automated)

#### 6. Visitor Statistics 📊
- [ ] **Visitor tracking implementation**
  - [ ] Cookie-based or IP-based tracking (GDPR compliant)
  - [ ] Session tracking
  - [ ] Privacy policy integration
- [ ] **Statistics dashboard**
  - [ ] Daily visitor count
  - [ ] Monthly visitor count
  - [ ] Annual visitor count
  - [ ] Export functionality for reports
- [ ] **Admin analytics panel**
  - [ ] Charts and graphs
  - [ ] Date range filtering
  - [ ] Export to CSV/PDF

### 🎨 Additional Features

#### 7. General Website Features
- [ ] **Multi-language support** (French, Italian, English, German - for Swiss context)
- [ ] **Responsive design** for mobile, tablet, desktop
- [ ] **Accessibility** (WCAG compliance)
- [ ] **SEO optimization**
- [ ] **Cookie consent banner** (GDPR compliance)
- [ ] **Privacy policy page**
- [ ] **Terms and conditions page**
- [ ] **Contact page**
- [ ] **Events and promotions section**
- [ ] **Social media integration**
- [ ] **Newsletter subscription**

#### 8. Services & Amenities Information
- [ ] **Display information about**:
  - [ ] 9 bars and restaurants (from the plan)
  - [ ] Casino Admiral Mendrisio
  - [ ] The Sense Gallery
  - [ ] WiFi availability
  - [ ] Services (tailor, exchange office, tax refund, etc.)

### 📋 Project Management Requirements

Based on `documentation/Directives_v2.pdf`:

#### 9. Documentation (Deadline: May 1st, 2026)
- [ ] **Project planning documentation**
  - [ ] Planned schedule/timeline
  - [ ] Actual schedule (track deviations)
  - [ ] Budget breakdown
  - [ ] Feasibility study
  - [ ] Implementation phases
- [ ] **Individual documentation** (per team member)
  - [ ] Weekly work journals (due Thursday 17:00, PDF on Moodle)
  - [ ] Difficulties encountered
  - [ ] Improvements for future projects
- [ ] **Team documentation**
  - [ ] Weekly meeting minutes (procès-verbal, due Thursday 17:00, PDF on Moodle)
- [ ] **Client documentation**
  - [ ] User manual (for non-technical users)
  - [ ] Client description document
  - [ ] All client correspondence
  - [ ] Signed requirements document (cahier des charges)
  - [ ] Signed directives
- [ ] **Technical documentation**
  - [ ] Architecture documentation
  - [ ] Code documentation
  - [ ] Deployment guide
  - [ ] Maintenance guide
- [ ] **Presentation**
  - [ ] Final presentation to class
  - [ ] Demonstration of working application

### 🔧 Technical Infrastructure

#### 10. Technology Stack Decisions
- [ ] **Backend**
  - [ ] Database choice (PostgreSQL, SQLite, etc.)
  - [ ] API design and implementation
  - [ ] Server setup and deployment
- [ ] **Frontend**
  - [ ] Continue with Dioxus or consider alternatives
  - [ ] State management
  - [ ] Asset management
- [ ] **Hosting & Deployment**
  - [ ] Hosting provider selection (budget-conscious)
  - [ ] Domain name
  - [ ] SSL certificate
  - [ ] CI/CD pipeline
- [ ] **Third-party integrations**
  - [ ] Email service (for notifications)
  - [ ] Analytics (Google Analytics or alternative)
  - [ ] Parking API (if available)

### 📊 Progress Tracking

**Current Status**:
- Base Dioxus application structure ✅
- FakeStoreAPI integration (to be replaced) ⚠️
- SSR and routing implemented ✅
- Tailwind CSS styling ✅

**Next Steps**:
1. Design database schema for shopping center data
2. Implement authentication system
3. Create shop directory functionality
4. Design and implement the game system
5. Integrate shopping center map

---

**Note**: This project requires a significant pivot from a generic e-commerce demo to a specific shopping center information and engagement platform. Priority should be given to core client requirements: shop directory, interactive game, map integration, and CMS functionality.
