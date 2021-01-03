import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";

Vue.use(Vuex);

const apiServer = axios.create({
  baseURL: process.env.VUE_APP_AEC_BASE_URL,
});

export default new Vuex.Store({
  state: {
    routines: [],
  },
  mutations: {
    setRoutines(state, payload) {
      state.routines = payload.routines;
    },
  },
  actions: {
    async fetchRoutines(context) {
      const res = await apiServer.get("/routines", {
        responseType: "json",
      });
      context.commit("setRoutines", {
        routines: res.data,
      });
    },
    async addRoutine(context, payload) {
      const res = await apiServer.post(
        "/routines",
        payload.editedItem,
      );
    },
  },
});
