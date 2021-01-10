import Vue from "vue";
import Vuex from "vuex";
import routines from './modules/routines'

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    routines,
  }
});
