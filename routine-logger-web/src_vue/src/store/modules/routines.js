import RoutineAPI from "../../api/routines"

export default {
    namespaced: true,
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
            const routineList = await RoutineAPI.fetchRoutines()
            context.commit("setRoutines", {
                routines: routineList,
            });
        },
        async addRoutine(context, payload) {
            await RoutineAPI.addRoutine(payload.editedItem)
        },
        async updateRoutine(context, payload) {
            await RoutineAPI.updateRoutine(payload.editedItem)
        },
        async deleteRoutine(context, payload) {
            await RoutineAPI.deleteRoutine(payload.editedItem.routine_id)
        },
    },
}