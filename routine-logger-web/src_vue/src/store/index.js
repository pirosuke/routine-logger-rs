import Vue from "vue";
import Vuex from "vuex";
import routines from './modules/routines'
import routineLogs from './modules/routine_logs'

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    routines,
    routineLogs,
  }
});
