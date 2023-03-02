export const serializeForm = (form: HTMLFormElement) => {
	const formData = new FormData(form);
	const data: any = {};

	for (const field of formData) {
		const [key, value] = field;
		data[key] = value;
	}

	return data;
};
