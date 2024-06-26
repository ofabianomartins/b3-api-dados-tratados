import React from 'react'

const Dashboard = React.lazy(() => import('./views/dashboard/Dashboard'))
const Colors = React.lazy(() => import('./views/theme/colors/Colors'))
const Typography = React.lazy(() => import('./views/theme/typography/Typography'))

// Base
const Accordion = React.lazy(() => import('./views/base/accordion/Accordion'))
const Breadcrumbs = React.lazy(() => import('./views/base/breadcrumbs/Breadcrumbs'))
const Cards = React.lazy(() => import('./views/base/cards/Cards'))
const Carousels = React.lazy(() => import('./views/base/carousels/Carousels'))
const Collapses = React.lazy(() => import('./views/base/collapses/Collapses'))
const ListGroups = React.lazy(() => import('./views/base/list-groups/ListGroups'))
const Navs = React.lazy(() => import('./views/base/navs/Navs'))
const Paginations = React.lazy(() => import('./views/base/paginations/Paginations'))
const Placeholders = React.lazy(() => import('./views/base/placeholders/Placeholders'))
const Popovers = React.lazy(() => import('./views/base/popovers/Popovers'))
const Progress = React.lazy(() => import('./views/base/progress/Progress'))
const Spinners = React.lazy(() => import('./views/base/spinners/Spinners'))
const Tables = React.lazy(() => import('./views/base/tables/Tables'))
const Tooltips = React.lazy(() => import('./views/base/tooltips/Tooltips'))

// Buttons
const Buttons = React.lazy(() => import('./views/buttons/buttons/Buttons'))
const ButtonGroups = React.lazy(() => import('./views/buttons/button-groups/ButtonGroups'))
const Dropdowns = React.lazy(() => import('./views/buttons/dropdowns/Dropdowns'))

//Forms
const ChecksRadios = React.lazy(() => import('./views/forms/checks-radios/ChecksRadios'))
const FloatingLabels = React.lazy(() => import('./views/forms/floating-labels/FloatingLabels'))
const FormControl = React.lazy(() => import('./views/forms/form-control/FormControl'))
const InputGroup = React.lazy(() => import('./views/forms/input-group/InputGroup'))
const Layout = React.lazy(() => import('./views/forms/layout/Layout'))
const Range = React.lazy(() => import('./views/forms/range/Range'))
const Select = React.lazy(() => import('./views/forms/select/Select'))
const Validation = React.lazy(() => import('./views/forms/validation/Validation'))

// Icons
const CoreUIIcons = React.lazy(() => import('./views/icons/coreui-icons/CoreUIIcons'))
const Flags = React.lazy(() => import('./views/icons/flags/Flags'))
const Brands = React.lazy(() => import('./views/icons/brands/Brands'))

// Notifications
const Alerts = React.lazy(() => import('./views/notifications/alerts/Alerts'))
const Badges = React.lazy(() => import('./views/notifications/badges/Badges'))
const Modals = React.lazy(() => import('./views/notifications/modals/Modals'))
const Toasts = React.lazy(() => import('./views/notifications/toasts/Toasts'))

const Widgets = React.lazy(() => import('./views/widgets/Widgets'))

const CalendarList = React.lazy(() => import('./views/calendars/list'))
const CalendarCreate = React.lazy(() => import('./views/calendars/create'))
const SectorList = React.lazy(() => import('./views/sectors/list'))
const SectorCreate = React.lazy(() => import('./views/sectors/create'))
const CurrencyList = React.lazy(() => import('./views/currencies/list'))
const CurrencyShow = React.lazy(() => import('./views/currencies/show'))
const CurrencyCreate = React.lazy(() => import('./views/currencies/create'))
const CurrencyUpdate = React.lazy(() => import('./views/currencies/update'))

const SubsectorList = React.lazy(() => import('./views/subsectors/list'))
const SubsectorShow = React.lazy(() => import('./views/subsectors/show'))
const SubsectorCreate = React.lazy(() => import('./views/subsectors/create'))
const SubsectorUpdate = React.lazy(() => import('./views/subsectors/update'))

const CompanyList = React.lazy(() => import('./views/companies/list'))
const CompanyShow = React.lazy(() => import('./views/companies/show'))
const CompanyCreate = React.lazy(() => import('./views/companies/create'))
const CompanyUpdate = React.lazy(() => import('./views/companies/update'))

const TickerList = React.lazy(() => import('./views/tickers/list'))
const TickerShow = React.lazy(() => import('./views/tickers/show'))
const TickerCreate = React.lazy(() => import('./views/tickers/create'))
const TickerUpdate = React.lazy(() => import('./views/tickers/update'))

const IndicatorList = React.lazy(() => import('./views/indicators/list'))
const IndicatorShow = React.lazy(() => import('./views/indicators/show'))
const IndicatorCreate = React.lazy(() => import('./views/indicators/create'))
const IndicatorUpdate = React.lazy(() => import('./views/indicators/update'))

const QuoteList = React.lazy(() => import('./views/quotes/list'))

const TheoryPortfolioList = React.lazy(() => import('./views/theory_portfolios/list'))
const TheoryPortfolioTransactionList = React.lazy(() => import('./views/theory_portfolio_transactions/list'))

const routes = [
  { path: '/', exact: true, name: 'Home' },
  { path: '/dashboard', name: 'Dashboard', element: Dashboard },

  { path: '/calendars', name: 'Calendars', element: CalendarList },
  { path: '/calendars/create', name: 'Create Calendar', element: CalendarCreate },
  { path: '/sectors', name: 'Sectors', element: SectorList },
  { path: '/sectors/create', name: 'Create Sector', element: SectorCreate },
  { path: '/currencies', name: 'List Currencies', element: CurrencyList },
  { path: '/currencies/:id', name: 'Currency :id', element: CurrencyShow },
  { path: '/currencies/create', name: 'Create Currency', element: CurrencyCreate },
  { path: '/currencies/:id/edit', name: 'Edit Currency', element: CurrencyUpdate },

  { path: '/subsectors', name: 'List Subsectors', element: SubsectorList },
  { path: '/subsectors/:id', name: 'Subsector :id', element: SubsectorShow },
  { path: '/subsectors/create', name: 'Create Subsector', element: SubsectorCreate },
  { path: '/subsectors/:id/edit', name: 'Edit Subsector', element: SubsectorUpdate },

  { path: '/companies', name: 'List Companies', element: CompanyList },
  { path: '/companies/:id', name: 'Company :id', element: CompanyShow },
  { path: '/companies/create', name: 'Create Company', element: CompanyCreate },
  { path: '/companies/:id/edit', name: 'Edit Company', element: CompanyUpdate },

  { path: '/tickers', name: 'List Tickers', element: TickerList },
  { path: '/tickers/:id', name: 'Ticker :id', element: TickerShow },
  { path: '/tickers/create', name: 'Create Ticker', element: TickerCreate },
  { path: '/tickers/:id/edit', name: 'Edit Ticker', element: TickerUpdate },

  { path: '/indicators', name: 'List Indicators', element: IndicatorList },
  { path: '/indicators/:id', name: 'Indicator :id', element: IndicatorShow },
  { path: '/indicators/create', name: 'Create Indicator', element: IndicatorCreate },
  { path: '/indicators/:id/edit', name: 'Edit Indicator', element: IndicatorUpdate },

  { path: '/quotes', name: 'List Quotes', element: QuoteList },

  { path: '/theory_portfolios', name: 'List Theory Portfolios', element: TheoryPortfolioList },

  { path: '/theory_portfolio_transactions', name: 'List Theory Portfolio Transactionss', element: TheoryPortfolioTransactionList },

  { path: '/theme', name: 'Theme', element: Colors, exact: true },
  { path: '/theme/colors', name: 'Colors', element: Colors },
  { path: '/theme/typography', name: 'Typography', element: Typography },
  { path: '/base', name: 'Base', element: Cards, exact: true },
  { path: '/base/accordion', name: 'Accordion', element: Accordion },
  { path: '/base/breadcrumbs', name: 'Breadcrumbs', element: Breadcrumbs },
  { path: '/base/cards', name: 'Cards', element: Cards },
  { path: '/base/carousels', name: 'Carousel', element: Carousels },
  { path: '/base/collapses', name: 'Collapse', element: Collapses },
  { path: '/base/list-groups', name: 'List Groups', element: ListGroups },
  { path: '/base/navs', name: 'Navs', element: Navs },
  { path: '/base/paginations', name: 'Paginations', element: Paginations },
  { path: '/base/placeholders', name: 'Placeholders', element: Placeholders },
  { path: '/base/popovers', name: 'Popovers', element: Popovers },
  { path: '/base/progress', name: 'Progress', element: Progress },
  { path: '/base/spinners', name: 'Spinners', element: Spinners },
  { path: '/base/tables', name: 'Tables', element: Tables },
  { path: '/base/tooltips', name: 'Tooltips', element: Tooltips },
  { path: '/buttons', name: 'Buttons', element: Buttons, exact: true },
  { path: '/buttons/buttons', name: 'Buttons', element: Buttons },
  { path: '/buttons/dropdowns', name: 'Dropdowns', element: Dropdowns },
  { path: '/buttons/button-groups', name: 'Button Groups', element: ButtonGroups },
  { path: '/forms', name: 'Forms', element: FormControl, exact: true },
  { path: '/forms/form-control', name: 'Form Control', element: FormControl },
  { path: '/forms/select', name: 'Select', element: Select },
  { path: '/forms/checks-radios', name: 'Checks & Radios', element: ChecksRadios },
  { path: '/forms/range', name: 'Range', element: Range },
  { path: '/forms/input-group', name: 'Input Group', element: InputGroup },
  { path: '/forms/floating-labels', name: 'Floating Labels', element: FloatingLabels },
  { path: '/forms/layout', name: 'Layout', element: Layout },
  { path: '/forms/validation', name: 'Validation', element: Validation },
  { path: '/icons', exact: true, name: 'Icons', element: CoreUIIcons },
  { path: '/icons/coreui-icons', name: 'CoreUI Icons', element: CoreUIIcons },
  { path: '/icons/flags', name: 'Flags', element: Flags },
  { path: '/icons/brands', name: 'Brands', element: Brands },
  { path: '/notifications', name: 'Notifications', element: Alerts, exact: true },
  { path: '/notifications/alerts', name: 'Alerts', element: Alerts },
  { path: '/notifications/badges', name: 'Badges', element: Badges },
  { path: '/notifications/modals', name: 'Modals', element: Modals },
  { path: '/notifications/toasts', name: 'Toasts', element: Toasts },
  { path: '/widgets', name: 'Widgets', element: Widgets },
]

export default routes
