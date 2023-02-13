import { initializeApp } from 'firebase/app';
import { getFirestore, connectFirestoreEmulator } from 'firebase/firestore';
import {
	getAuth,
	connectAuthEmulator,
	onAuthStateChanged,
	signInWithEmailAndPassword,
	signOut as _signOut,
	type User
} from 'firebase/auth';
import { writable } from 'svelte/store';

// Initialize Firebase
const app = initializeApp({
	projectId: 'event-001-bb028',
	apiKey: 'xxx'
});

const _db = getFirestore(app);
const _auth = getAuth(app);

connectFirestoreEmulator(_db, 'localhost', 8080);
connectAuthEmulator(_auth, 'http://localhost:9099');

export const db = _db;
export const auth = _auth;

export const signOut = async () => {
	await _signOut(auth);
};
export const signInWithEmailAndPassword$ = async (email: string, password: string) => {
	return await signInWithEmailAndPassword(auth, email, password);
};

/**
 * @param  {Auth} auth firebase auth instance
 * @returns a store with the current firebase user
 */
export function userStore() {
	let unsubscribe: () => void;

	if (!auth || !globalThis.window) {
		console.warn('Auth is not initialized on not in browser');
		const { subscribe } = writable(undefined);
		return {
			subscribe
		};
	}

	const { subscribe } = writable<undefined | null | User>(undefined, (set) => {
		unsubscribe = onAuthStateChanged(auth, (user) => {
			set(user);
		});

		return () => unsubscribe();
	});

	return {
		subscribe
	};
}
