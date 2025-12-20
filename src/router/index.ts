import { createRouter, createWebHistory } from 'vue-router';
import CalendarView from '@/views/CalendarView.vue';
import EventView from '@/views/EventView.vue';
import SettingsView from '@/views/SettingsView.vue';

const routes = [
  {
    path: '/',
    name: 'Calendar',
    component: CalendarView,
  },
  {
    path: '/events',
    name: 'Events',
    component: EventView,
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
